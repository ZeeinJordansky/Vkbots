use std::{env, net::SocketAddr};

use vkbots::{callback::CallbackConfig, VkBot};

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let token = env::var("VK_TOKEN").expect("Укажите VK_TOKEN");
    let group_id: i64 = env::var("VK_GROUP_ID")
        .expect("Укажите VK_GROUP_ID")
        .parse()
        .expect("VK_GROUP_ID должен быть числом");
    let secret = env::var("VK_CALLBACK_SECRET").expect("Укажите VK_CALLBACK_SECRET");
    let confirmation = env::var("VK_CONFIRMATION_KEY")
        .expect("Укажите VK_CONFIRMATION_KEY");

    let bot = VkBot::new(token);
    let mut dispatcher = bot.dispatcher();

    dispatcher.command("/start", |context| async move {
        context.answer("Бот работает через Callback API.").await?;
        Ok(())
    });

    dispatcher
        .callback_server(
            CallbackConfig::new(group_id)
                .secret(secret)
                .confirmation_key(confirmation),
        )
        .serve("127.0.0.1:3000".parse::<SocketAddr>().unwrap())
        .await
}
