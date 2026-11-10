use vkbots::{dispatcher::filter, VkBot};

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let token = std::env::var("VK_TOKEN").expect("Укажите VK_TOKEN");
    let group_id: i64 = std::env::var("VK_GROUP_ID")
        .expect("Укажите VK_GROUP_ID")
        .parse()
        .expect("VK_GROUP_ID должен быть числом");

    let bot = VkBot::new(token);
    let mut dispatcher = bot.dispatcher();

    dispatcher.message(filter::contains("бот, привет"), |context| async move {
        if let Some(message) = &context.message {
            let peer_id = message.peer_id.unwrap_or_default();
            let user_id = message.from_id.unwrap_or_default();
            context.bot.send
                .to(peer_id)
                .text(format!("Привет, пользователь {user_id}!"))
                .send()
                .await?;
        }
        Ok(())
    });

    dispatcher.run_long_poll(group_id).await
}
