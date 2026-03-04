mod autostart;
mod bot;
mod config;
mod http;
mod system_manager;

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[tokio::main]
async fn main() {
    // Обработка аргументов командной строки до инициализации логгера
    let args: Vec<String> = std::env::args().collect();
    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "--install" => {
                autostart::install();
                return;
            }
            "--uninstall" => {
                autostart::uninstall();
                return;
            }
            unknown => {
                eprintln!("Неизвестный аргумент: {}", unknown);
                eprintln!("Использование: remote_commander [--install | --uninstall]");
                std::process::exit(1);
            }
        }
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("remote_commander=info".parse().unwrap()),
        )
        .init();

    let config = match config::Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Ошибка конфигурации: {e}");
            eprintln!("Создай файл .env на основе .env.example и заполни значения.");
            std::process::exit(1);
        }
    };

    let manager = Arc::new(Mutex::new(system_manager::SystemManager::new()));

    info!(
        "Remote Commander запущен. IDs: {:?}, HTTP порт: {}",
        config.allowed_chat_ids, config.http_port
    );

    tokio::join!(
        bot::run(config.bot_token, config.allowed_chat_ids, Arc::clone(&manager)),
        http::run(config.http_port, Arc::clone(&manager)),
    );
}
