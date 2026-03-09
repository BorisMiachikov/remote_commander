# Roadmap: Разработка Remote Commander (Rust)

Поэтапная история создания кроссплатформенного приложения для удалённого управления ПК через Telegram-бот.

---

## ✅ Этап 1: Фундамент и ядро (Core Logic)

**Цель:** реализовать базовые системные функции без интерфейсов.

- [x] Инициализация проекта: `cargo new remote_commander`.
- [x] Конфигурация зависимостей: `tokio`, `sysinfo`, `system_shutdown`, `webbrowser` в `Cargo.toml`.
- [x] Модуль `SystemManager` с функциями:
    - `shutdown()` — выключение через `system_shutdown`.
    - `open_url(url)` — открытие браузера через `webbrowser`.
    - `get_metrics()` — сбор загрузки CPU и RAM через `sysinfo`.

---

## ✅ Этап 2: Интерфейс Telegram (MVP)

**Цель:** управление через чат-бота с минимальной безопасностью.

- [x] Регистрация бота в BotFather, настройка `teloxide`.
- [x] Whitelist по `ALLOWED_CHAT_IDS` — бот игнорирует чужие запросы.
- [x] Команды `/status`, `/youtube`, `/poweroff` (с inline-подтверждением).
- [x] Корректная обработка ошибок с ответом пользователю.

---

## ✅ Этап 3: Расширенное управление (процессы и аудио)

**Цель:** добавление контроля над процессами и звуком.

- [x] `kill_process(name)` — поиск и завершение процессов по имени через `sysinfo`.
- [x] `set_volume(level)` — Windows: Core Audio COM API (`windows` 0.52); Linux: `amixer`.
- [x] Команды `/kill <имя>` и `/vol <уровень>` в боте.

---

## ✅ Этап 4: Автономность и деплой

**Цель:** сделать приложение частью системы.

- [x] Конфигурация через `.env` (токен, whitelist, лог-уровень).
- [x] Автозапуск Windows: Task Scheduler (`schtasks /sc ONLOGON`).
- [x] Автозапуск Linux: systemd user unit.
- [x] CLI-аргументы `--install` / `--uninstall`.
- [x] Оптимизированная release-сборка (~6 МБ).

---

## ✅ Этап 5: Улучшение Telegram-бота

**Цель:** расширить набор команд и убрать неиспользуемые интерфейсы.

- [x] Удалён HTTP API (Axum) — управление только через Telegram.
- [x] Добавлена команда `/reboot` с inline-подтверждением.
- [x] Добавлена команда `/help` — список всех команд с описанием.

---

## Технологический стек

| | |
|-|-|
| Язык | Rust 2021 |
| Async runtime | tokio |
| Telegram | teloxide 0.12 |
| Мониторинг | sysinfo 0.30 |
| Питание | system_shutdown |
| Браузер | webbrowser |
| Аудио (Windows) | windows 0.52 (Core Audio COM API) |
| Аудио (Linux) | amixer (subprocess) |
| Конфиг | dotenvy (.env) |
| Логирование | tracing + tracing-subscriber |
| Ошибки | anyhow |
