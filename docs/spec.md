# Техническое задание: Дистанционный контроллер ПК на Rust (Remote Commander)

## 1. Общее описание
Консольное приложение на языке Rust для удалённого управления компьютером через Telegram-бот. Приложение кроссплатформенное (Windows/Linux) и обладает модульной архитектурой. Поддерживает автозапуск при входе в систему.

## 2. Стек технологий
- **Язык:** Rust (Edition 2021)
- **Async Runtime:** `tokio`
- **Управление системой:**
    - `sysinfo` (мониторинг CPU/RAM, управление процессами)
    - `system_shutdown` (выключение/перезагрузка)
    - `webbrowser` (открытие ссылок)
    - `windows` 0.52 (Core Audio COM API для громкости, только Windows)
    - `amixer` (управление громкостью, только Linux, вызов через subprocess)
- **Интерфейс:** `teloxide` (Telegram Bot API)
- **Конфигурация:** `dotenvy` (загрузка `.env`)
- **Логирование:** `tracing` + `tracing-subscriber`
- **Обработка ошибок:** `anyhow`

## 3. Функциональные требования

### 3.1. Модуль Core (Системные команды)
Структура `SystemManager`, предоставляющая методы:
- `get_metrics()` → `SystemStats { cpu_usage, used_memory, total_memory }` — текущая загрузка CPU и RAM.
- `kill_process(name: &str)` → `Result<String>` — поиск и завершение всех процессов с указанным именем.
- `open_url(url: &str)` → `Result<()>` — открытие ссылки в браузере по умолчанию.
- `set_volume(level: u16)` → `Result<()>` — установка уровня громкости (0–100).
- `shutdown()` → `Result<()>` — полное выключение ПК.
- `reboot()` → `Result<()>` — перезагрузка ПК.

### 3.2. Модуль Telegram-бота
- Реализован через `teloxide` с диспетчером `dptree`.
- **Безопасность:** whitelist по `ALLOWED_CHAT_IDS` — бот игнорирует все запросы от неизвестных chat_id.
- **Команды:**
    - `/help` — список всех доступных команд.
    - `/status` — загрузка CPU и RAM.
    - `/youtube` — открыть YouTube в браузере.
    - `/kill <имя>` — завершить процесс по имени.
    - `/vol <0-100>` — установить уровень громкости.
    - `/poweroff` — выключить компьютер (с подтверждением через inline-кнопки).
    - `/reboot` — перезагрузить компьютер (с подтверждением через inline-кнопки).

### 3.3. Автозапуск
- **Windows:** регистрация задачи в Task Scheduler (`schtasks /sc ONLOGON`).
- **Linux:** запись systemd user unit (`~/.config/systemd/user/remote_commander.service`).
- Управление через аргументы командной строки: `--install` / `--uninstall`.

## 4. Архитектура проекта

```
main.rs          — точка входа: CLI-аргументы, инициализация, запуск бота
config.rs        — загрузка .env (TELEGRAM_BOT_TOKEN, ALLOWED_CHAT_IDS)
system_manager.rs — SystemManager: все системные операции
bot.rs           — Telegram-бот: команды, whitelist, inline-кнопки
autostart.rs     — регистрация/удаление автозапуска
```

`SystemManager` создаётся в `main.rs` как `Arc<Mutex<SystemManager>>` и передаётся в бота.

## 5. Конфигурация

Файл `.env` рядом с исполняемым файлом (или в рабочей директории):
```env
TELEGRAM_BOT_TOKEN=...
ALLOWED_CHAT_IDS=123456789,987654321
RUST_LOG=remote_commander=info
```

## 6. Требования к реализации
1. Асинхронный код через `tokio` везде, где это уместно.
2. Без `unwrap()` в продакшн-логике — использовать `anyhow::Result` и `?`.
3. Логирование через `tracing`.
4. Оптимизированная release-сборка: LTO, `codegen-units=1`, `strip=true` (~6 МБ на Windows).
