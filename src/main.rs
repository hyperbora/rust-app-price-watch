use dotenv;
use teloxide::prelude::*;
use tokio;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |u: UpdateWithCx<Bot, Message>| async move {
        u.requester.send_dice(u.update.chat.id).send().await?;
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await;

    Ok(())
}
