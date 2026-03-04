use std::sync::Arc;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::system_manager::SystemManager;

type SharedManager = Arc<Mutex<SystemManager>>;
type AllowedIds = Arc<Vec<i64>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды Remote Commander:")]
enum Command {
    #[command(description = "Статус системы (CPU и RAM)")]
    Status,
    #[command(description = "Открыть YouTube в браузере")]
    Youtube,
    #[command(description = "Завершить процесс: /kill <имя>")]
    Kill(String),
    #[command(description = "Установить громкость: /vol <0-100>")]
    Vol(String),
    #[command(description = "Выключить компьютер (запросит подтверждение)")]
    Poweroff,
}

pub async fn run(token: String, allowed_ids: Vec<i64>, manager: SharedManager) {
    let bot = Bot::new(token);
    let allowed_ids: AllowedIds = Arc::new(allowed_ids);

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(handle_command),
        )
        .branch(Update::filter_callback_query().endpoint(handle_callback));

    info!("Бот запущен и ожидает команды...");

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![manager, allowed_ids])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    manager: SharedManager,
    allowed_ids: AllowedIds,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    if !allowed_ids.contains(&chat_id.0) {
        warn!("Запрос от неизвестного chat_id={}", chat_id.0);
        return Ok(());
    }

    match cmd {
        Command::Status => {
            info!("Команда /status от {}", chat_id.0);
            let stats = manager.lock().await.get_metrics();
            bot.send_message(
                chat_id,
                format!(
                    "CPU: {:.1}%\nRAM: {} МБ / {} МБ",
                    stats.cpu_usage, stats.used_memory, stats.total_memory
                ),
            )
            .await?;
        }

        Command::Youtube => {
            info!("Команда /youtube от {}", chat_id.0);
            let result = manager.lock().await.open_url("https://www.youtube.com");
            let text = match result {
                Ok(_) => "YouTube открыт в браузере.".to_string(),
                Err(e) => format!("Ошибка: {e}"),
            };
            bot.send_message(chat_id, text).await?;
        }

        Command::Kill(name) => {
            let name = name.trim().to_string();
            if name.is_empty() {
                bot.send_message(chat_id, "Использование: /kill <имя_процесса>")
                    .await?;
                return Ok(());
            }
            info!("Команда /kill '{}' от {}", name, chat_id.0);
            let result = manager.lock().await.kill_process(&name);
            let text = match result {
                Ok(msg) => msg,
                Err(e) => format!("Ошибка: {e}"),
            };
            bot.send_message(chat_id, text).await?;
        }

        Command::Vol(arg) => {
            let arg = arg.trim().to_string();
            match arg.parse::<u16>() {
                Ok(level) if level <= 100 => {
                    info!("Команда /vol {} от {}", level, chat_id.0);
                    let result = manager.lock().await.set_volume(level);
                    let text = match result {
                        Ok(_) => format!("Громкость установлена: {}%", level),
                        Err(e) => format!("Ошибка: {e}"),
                    };
                    bot.send_message(chat_id, text).await?;
                }
                _ => {
                    bot.send_message(chat_id, "Использование: /vol <0-100>")
                        .await?;
                }
            }
        }

        Command::Poweroff => {
            info!("Команда /poweroff от {}", chat_id.0);
            let keyboard = InlineKeyboardMarkup::new([[
                InlineKeyboardButton::callback("✅ Да, выключить", "confirm_poweroff"),
                InlineKeyboardButton::callback("❌ Отмена", "cancel_poweroff"),
            ]]);
            bot.send_message(chat_id, "Выключить компьютер?")
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}

async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    manager: SharedManager,
    allowed_ids: AllowedIds,
) -> ResponseResult<()> {
    let chat_id = match q.message.as_ref().map(|m| m.chat.id) {
        Some(id) => id,
        None => {
            bot.answer_callback_query(q.id).await?;
            return Ok(());
        }
    };

    if !allowed_ids.contains(&chat_id.0) {
        bot.answer_callback_query(q.id).await?;
        return Ok(());
    }

    bot.answer_callback_query(&q.id).await?;

    match q.data.as_deref() {
        Some("confirm_poweroff") => {
            info!("Подтверждение выключения от {}", chat_id.0);
            if let Some(msg) = &q.message {
                bot.edit_message_text(chat_id, msg.id, "Выключаю компьютер...")
                    .await?;
            }
            if let Err(e) = manager.lock().await.shutdown() {
                bot.send_message(chat_id, format!("Ошибка выключения: {e}"))
                    .await?;
            }
        }

        Some("cancel_poweroff") => {
            info!("Выключение отменено пользователем {}", chat_id.0);
            if let Some(msg) = &q.message {
                bot.edit_message_text(chat_id, msg.id, "Отменено.").await?;
            }
        }

        _ => {}
    }

    Ok(())
}
