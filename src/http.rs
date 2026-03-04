use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::system_manager::SystemManager;

type SharedManager = Arc<Mutex<SystemManager>>;

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
    arg: Option<String>,
}

#[derive(Serialize)]
struct CommandResponse {
    ok: bool,
    message: String,
}

impl CommandResponse {
    fn ok(message: impl Into<String>) -> Json<Self> {
        Json(Self { ok: true, message: message.into() })
    }

    fn err(message: impl Into<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::BAD_REQUEST,
            Json(Self { ok: false, message: message.into() }),
        )
    }
}

pub async fn run(port: u16, manager: SharedManager) {
    let app = Router::new()
        .route("/api/command", post(handle_command))
        .with_state(manager);

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Не удалось запустить HTTP сервер");

    info!("HTTP API запущен на http://{}/api/command", addr);

    axum::serve(listener, app)
        .await
        .expect("HTTP сервер завершился с ошибкой");
}

async fn handle_command(
    State(manager): State<SharedManager>,
    Json(req): Json<CommandRequest>,
) -> impl IntoResponse {
    let cmd = req.command.trim().to_lowercase();
    let arg = req.arg.as_deref().unwrap_or("").trim().to_string();

    info!("HTTP команда: '{}' arg='{}'", cmd, arg);

    match cmd.as_str() {
        "status" => {
            let stats = manager.lock().await.get_metrics();
            let msg = format!(
                "CPU: {:.1}%\nRAM: {} МБ / {} МБ",
                stats.cpu_usage, stats.used_memory, stats.total_memory
            );
            CommandResponse::ok(msg).into_response()
        }

        "youtube" => match manager.lock().await.open_url("https://www.youtube.com") {
            Ok(_) => CommandResponse::ok("YouTube открыт в браузере.").into_response(),
            Err(e) => CommandResponse::err(format!("Ошибка: {e}")).into_response(),
        },

        "kill" => {
            if arg.is_empty() {
                return CommandResponse::err("Укажи имя процесса: {\"command\":\"kill\",\"arg\":\"name.exe\"}").into_response();
            }
            match manager.lock().await.kill_process(&arg) {
                Ok(msg) => CommandResponse::ok(msg).into_response(),
                Err(e) => CommandResponse::err(format!("Ошибка: {e}")).into_response(),
            }
        }

        "vol" => {
            match arg.parse::<u16>() {
                Ok(level) if level <= 100 => {
                    match manager.lock().await.set_volume(level) {
                        Ok(_) => CommandResponse::ok(format!("Громкость: {}%", level)).into_response(),
                        Err(e) => CommandResponse::err(format!("Ошибка: {e}")).into_response(),
                    }
                }
                _ => CommandResponse::err("Укажи уровень 0-100: {\"command\":\"vol\",\"arg\":\"50\"}").into_response(),
            }
        }

        "poweroff" => match manager.lock().await.shutdown() {
            Ok(_) => CommandResponse::ok("Выключение...").into_response(),
            Err(e) => CommandResponse::err(format!("Ошибка: {e}")).into_response(),
        },

        unknown => {
            warn!("Неизвестная HTTP команда: '{}'", unknown);
            CommandResponse::err(format!("Неизвестная команда: '{}'", unknown)).into_response()
        }
    }
}
