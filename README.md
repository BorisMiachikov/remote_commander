# Remote Commander

Приложение на Rust для удалённого управления компьютером через Telegram-бот и локальный HTTP API.

## Возможности

- **Telegram-бот** — управление через личный чат, защита whitelist по chat_id
- **HTTP API** — `POST /api/command` для интеграции со скриптами и другими ботами
- **Мониторинг** — загрузка CPU и использование RAM
- **Управление процессами** — завершение процесса по имени
- **Громкость** — установка уровня системного звука (0–100)
- **Браузер** — открытие YouTube одной командой
- **Питание** — выключение с подтверждением
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
HTTP_PORT=3000
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
| `/status` | CPU % и RAM (МБ) |
| `/youtube` | Открыть YouTube в браузере |
| `/kill <имя>` | Завершить процесс, например `/kill chrome.exe` |
| `/vol <0-100>` | Установить громкость, например `/vol 50` |
| `/poweroff` | Выключить компьютер (с подтверждением) |

---

## HTTP API

**Endpoint:** `POST http://localhost:3000/api/command`

**Запрос:**
```json
{"command": "...", "arg": "..."}
```

**Ответ:**
```json
{"ok": true, "message": "..."}
```

| `command` | `arg` | Описание |
|-----------|-------|----------|
| `status` | — | Метрики CPU и RAM |
| `youtube` | — | Открыть YouTube |
| `kill` | имя процесса | Завершить процесс |
| `vol` | 0–100 | Установить громкость |
| `poweroff` | — | Выключить компьютер |

**Примеры:**
```bash
curl -X POST http://localhost:3000/api/command \
  -H "Content-Type: application/json" \
  -d '{"command":"status"}'

curl -X POST http://localhost:3000/api/command \
  -d '{"command":"kill","arg":"chrome.exe"}'

curl -X POST http://localhost:3000/api/command \
  -d '{"command":"vol","arg":"40"}'
```

---

## Стек

| | |
|-|-|
| Язык | Rust 2021 |
| Async runtime | tokio |
| Telegram | teloxide 0.12 |
| HTTP | axum 0.7 |
| Система | sysinfo, system_shutdown, webbrowser |
| Аудио (Windows) | Windows Core Audio COM API |
| Конфиг | dotenvy (.env) |
