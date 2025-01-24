use dotenv::dotenv;
use teloxide::prelude::*;
use tokio;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> HandlerResult {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |cx: UpdateWithCx<Bot, Message>| async move {
        if let Some(text) = cx.update.text() {
            cx.answer(format!("You said: {}", text)).send().await?;
        }
        HandlerResult::Ok(())
    })
    .await;

    Ok(())
}
