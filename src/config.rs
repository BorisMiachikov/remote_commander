use anyhow::{Context, Result};

pub struct Config {
    pub bot_token: String,
    pub allowed_chat_ids: Vec<i64>,
    pub http_port: u16,
}

impl Config {
    pub fn load() -> Result<Self> {
        // Сначала ищем .env рядом с exe (для автозапуска),
        // потом в текущей директории (для разработки).
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                dotenvy::from_path(dir.join(".env")).ok();
            }
        }
        dotenvy::dotenv().ok();

        let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
            .context("Переменная TELEGRAM_BOT_TOKEN не задана в .env")?;

        let ids_str = std::env::var("ALLOWED_CHAT_IDS")
            .context("Переменная ALLOWED_CHAT_IDS не задана в .env")?;

        let allowed_chat_ids = ids_str
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                s.trim()
                    .parse::<i64>()
                    .context("Неверный формат ALLOWED_CHAT_IDS (ожидаются числа через запятую)")
            })
            .collect::<Result<Vec<_>>>()?;

        let http_port = std::env::var("HTTP_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(3000);

        Ok(Self {
            bot_token,
            allowed_chat_ids,
            http_port,
        })
    }
}
