# Remote Commander

Приложение на Rust для удалённого управления компьютером через Telegram-бот.

## Возможности

- **Telegram-бот** — управление через личный чат, защита whitelist по chat_id
- **Мониторинг** — загрузка CPU и использование RAM
- **Управление процессами** — завершение процесса по имени
- **Громкость** — установка уровня системного звука (0–100)
- **Браузер** — открытие YouTube одной командой
- **Питание** — выключение и перезагрузка с подтверждением
- **Автозапуск** — Task Scheduler (Windows) / systemd (Linux)

## Быстрый старт

### 1. Получить токен бота

Напиши [@BotFather](https://t.me/BotFather) → `/newbot`, сохрани токен.

Узнай свой chat_id через [@userinfobot](https://t.me/userinfobot).

### 2. Настроить конфигурацию

```bash
cp .env.example .env
```

Заполни `.env`:
```env
TELEGRAM_BOT_TOKEN=1234567890:AAFxxxxxxxxxxxxxxxxxxxx
ALLOWED_CHAT_IDS=123456789
RUST_LOG=remote_commander=info
```

### 3. Собрать и запустить

```bash
cargo build --release
./target/release/remote_commander
```

### 4. Автозапуск (опционально)

**Windows** (от имени Администратора):
```bash
./target/release/remote_commander.exe --install
```

**Linux:**
```bash
./target/release/remote_commander --install
systemctl --user daemon-reload
systemctl --user enable --now remote_commander
```

Удалить из автозапуска: `remote_commander --uninstall`

---

## Telegram-команды

| Команда | Описание |
|---------|----------|
| `/help` | Список всех команд |
| `/status` | CPU % и RAM (МБ) |
| `/youtube` | Открыть YouTube в браузере |
| `/kill <имя>` | Завершить процесс, например `/kill chrome.exe` |
| `/vol <0-100>` | Установить громкость, например `/vol 50` |
| `/poweroff` | Выключить компьютер (с подтверждением) |
| `/reboot` | Перезагрузить компьютер (с подтверждением) |

---

## Стек

| | |
|-|-|
| Язык | Rust 2021 |
| Async runtime | tokio |
| Telegram | teloxide 0.12 |
| Мониторинг | sysinfo 0.30 |
| Аудио (Windows) | Windows Core Audio COM API |
| Аудио (Linux) | amixer |
| Конфиг | dotenvy (.env) |
