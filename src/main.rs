mod evedge;
mod evedge_api;

use std::time::Duration;

use evedge_api::EvedgeChargingStation;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Entrypoint for this bot.")]
    Start,
    #[command(description = "Subscribe to notifications about a charging station.")]
    WatchCharger(String),
    #[command(description = "display this text.")]
    Help,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome to the EV Charging Notifications Bot!")
                .await?;

            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;

            Ok(())
        }
        Command::WatchCharger(charging_station_id) => {
            if charging_station_id.is_empty() {
                bot.send_message(msg.chat.id, "Please provide a charging station ID.")
                    .await?;

                bot.send_message(msg.chat.id, "Example:").await?;
                bot.send_message(msg.chat.id, "/watchcharger 597122")
                    .await?;
                return Ok(());
            }

            let chat_id = msg.chat.id;
            let station = evedge::fetch_station_by_id(&charging_station_id).await?;

            tokio::spawn(async move {
                let start_time = tokio::time::Instant::now();
                let end_time = start_time + Duration::from_secs(60 * 15); // default timeout is 15 minutes
                let mut interval = tokio::time::interval_at(start_time, Duration::from_secs(10));

                while tokio::time::Instant::now() < end_time {
                    interval.tick().await;

                    // TODO: check for changes in chargers
                    let _ = send_station_details(bot.clone(), chat_id, &station).await;
                }
            });

            Ok(())
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;

            Ok(())
        }
    }
}

async fn send_station_details(
    bot: Bot,
    chat_id: ChatId,
    station: &EvedgeChargingStation,
) -> ResponseResult<()> {
    bot.send_message(
        chat_id,
        format!(
            "Name: {}\n\rConnectors Available: {}/{}",
            station.friendly_name.trim(),
            station.available_evses,
            station.total_evses
        ),
    )
    .await?;

    Ok(())
}
