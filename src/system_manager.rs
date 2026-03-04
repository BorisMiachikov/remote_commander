use sysinfo::{Pid, System};
use anyhow::{Context, Result};

pub struct SystemStats {
    pub cpu_usage: f32,
    pub used_memory: u64,
    pub total_memory: u64,
}

pub struct SystemManager {
    sys: System,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    /// Получить текущие метрики системы (Загрузка CPU и RAM)
    pub fn get_metrics(&mut self) -> SystemStats {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        let cpu_usage = self.sys.global_cpu_info().cpu_usage();
        let used_memory = self.sys.used_memory() / 1024 / 1024; // в МБ
        let total_memory = self.sys.total_memory() / 1024 / 1024; // в МБ

        SystemStats {
            cpu_usage,
            used_memory,
            total_memory,
        }
    }

    /// Найти и завершить все процессы с указанным именем
    pub fn kill_process(&mut self, name: &str) -> Result<String> {
        self.sys.refresh_processes();

        let name_lower = name.to_lowercase();
        let pids: Vec<Pid> = self
            .sys
            .processes()
            .iter()
            .filter(|(_, p)| p.name().to_lowercase() == name_lower)
            .map(|(pid, _)| *pid)
            .collect();

        if pids.is_empty() {
            return Ok(format!("Процесс '{}' не найден", name));
        }

        let mut killed = 0;
        for pid in &pids {
            if let Some(process) = self.sys.process(*pid) {
                if process.kill() {
                    killed += 1;
                }
            }
        }

        Ok(format!(
            "Завершено: {}/{} процессов '{}'",
            killed,
            pids.len(),
            name
        ))
    }

    /// Открыть YouTube или любую ссылку
    pub fn open_url(&self, url: &str) -> Result<()> {
        webbrowser::open(url).context("Не удалось открыть браузер")?;
        Ok(())
    }

    /// Установить системную громкость (0-100)
    pub fn set_volume(&self, level: u16) -> Result<()> {
        volume::set(level.min(100))
    }

    /// Выключить компьютер
    pub fn shutdown(&self) -> Result<()> {
        system_shutdown::shutdown().context("Ошибка при попытке выключения ПК")?;
        Ok(())
    }

    /// Перезагрузка
    pub fn reboot(&self) -> Result<()> {
        system_shutdown::reboot().context("Ошибка при попытке перезагрузки ПК")?;
        Ok(())
    }
}

#[cfg(windows)]
mod volume {
    use anyhow::{Context, Result};
    use windows::Win32::{
        Media::Audio::{
            eConsole, eRender,
            Endpoints::IAudioEndpointVolume,
            IMMDeviceEnumerator, MMDeviceEnumerator,
        },
        System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED},
    };

    pub fn set(level: u16) -> Result<()> {
        unsafe {
            // Игнорируем ошибку: COM может быть уже инициализирован
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

            let enumerator: IMMDeviceEnumerator =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
                    .context("Не удалось создать MMDeviceEnumerator")?;

            let device = enumerator
                .GetDefaultAudioEndpoint(eRender, eConsole)
                .context("Не удалось получить аудиоустройство")?;

            let endpoint: IAudioEndpointVolume = device
                .Activate(CLSCTX_ALL, None)
                .context("Не удалось активировать IAudioEndpointVolume")?;

            endpoint
                .SetMasterVolumeLevelScalar(level as f32 / 100.0, std::ptr::null())
                .context("Не удалось установить громкость")?;
        }
        Ok(())
    }
}

#[cfg(not(windows))]
mod volume {
    use anyhow::Result;

    pub fn set(level: u16) -> Result<()> {
        // Linux: amixer или pactl
        let status = std::process::Command::new("amixer")
            .args(["set", "Master", &format!("{}%", level)])
            .status()
            .map_err(|e| anyhow::anyhow!("amixer недоступен: {e}"))?;

        if !status.success() {
            anyhow::bail!("amixer вернул ошибку");
        }
        Ok(())
    }
}
