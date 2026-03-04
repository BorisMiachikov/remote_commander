/// Зарегистрировать приложение в автозапуске.
pub fn install() {
    #[cfg(windows)]
    install_windows();

    #[cfg(not(windows))]
    install_linux();
}

/// Удалить приложение из автозапуска.
pub fn uninstall() {
    #[cfg(windows)]
    uninstall_windows();

    #[cfg(not(windows))]
    uninstall_linux();
}

// ─── Windows ────────────────────────────────────────────────────────────────

#[cfg(windows)]
fn install_windows() {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Ошибка: не удалось определить путь к exe: {e}");
            std::process::exit(1);
        }
    };

    let exe_str = exe_path.to_string_lossy();

    let status = std::process::Command::new("schtasks")
        .args([
            "/create",
            "/tn", "RemoteCommander",
            "/tr", &format!("\"{}\"", exe_str),
            "/sc", "ONLOGON",
            "/f", // перезаписать если уже существует
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Автозапуск установлен (Планировщик задач).");
            println!("Задача: 'RemoteCommander' — запуск при входе в систему.");
            println!("Убедись, что .env находится рядом с exe: {}", exe_str);
        }
        Ok(_) => {
            eprintln!("schtasks завершился с ошибкой.");
            eprintln!("Попробуй запустить от имени Администратора.");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Не удалось запустить schtasks: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(windows)]
fn uninstall_windows() {
    let status = std::process::Command::new("schtasks")
        .args(["/delete", "/tn", "RemoteCommander", "/f"])
        .status();

    match status {
        Ok(s) if s.success() => println!("Задача 'RemoteCommander' удалена из Планировщика задач."),
        Ok(_) => {
            eprintln!("Задача не найдена или schtasks вернул ошибку.");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Не удалось запустить schtasks: {e}");
            std::process::exit(1);
        }
    }
}

// ─── Linux ──────────────────────────────────────────────────────────────────

#[cfg(not(windows))]
fn install_linux() {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Ошибка: {e}");
            std::process::exit(1);
        }
    };

    let exe_str = exe_path.display();
    let exe_dir = exe_path
        .parent()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "/opt/remote_commander".to_string());

    let service = format!(
        r#"[Unit]
Description=Remote Commander
After=network.target

[Service]
Type=simple
ExecStart={exe_str}
WorkingDirectory={exe_dir}
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
"#
    );

    let service_dir = dirs_linux();
    let service_path = format!("{}/remote_commander.service", service_dir);

    if let Err(e) = std::fs::create_dir_all(&service_dir) {
        eprintln!("Не удалось создать директорию {service_dir}: {e}");
        std::process::exit(1);
    }

    if let Err(e) = std::fs::write(&service_path, service) {
        eprintln!("Не удалось записать unit-файл: {e}");
        std::process::exit(1);
    }

    println!("Systemd unit записан: {}", service_path);
    println!();
    println!("Активируй автозапуск:");
    println!("  systemctl --user daemon-reload");
    println!("  systemctl --user enable --now remote_commander");
}

#[cfg(not(windows))]
fn uninstall_linux() {
    let service_path = format!("{}/remote_commander.service", dirs_linux());

    // Сначала останавливаем и отключаем
    let _ = std::process::Command::new("systemctl")
        .args(["--user", "disable", "--now", "remote_commander"])
        .status();

    match std::fs::remove_file(&service_path) {
        Ok(_) => {
            println!("Файл {} удалён.", service_path);
            println!("Запусти: systemctl --user daemon-reload");
        }
        Err(e) => {
            eprintln!("Не удалось удалить {service_path}: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(not(windows))]
fn dirs_linux() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    format!("{}/.config/systemd/user", home)
}
