use teloxide::prelude::*;
use tokio;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl_with_listener(
        bot.clone(),
        move |cx: UpdateWithCx<Bot, Message>| async move {
            cx.requester.send_dice(cx.update.chat.id).send().await?;
            Ok::<(), Error>(())
        },
        teloxide::dispatching::update_listeners::polling_default(bot).await,
    )
    .await;

    Ok(())
}
