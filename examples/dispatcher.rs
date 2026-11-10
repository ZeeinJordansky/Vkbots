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

    dispatcher.command("/start", |context| async move {
        context.answer("Привет! Я бот на Vkbots.").await?;
        Ok(())
    });

    dispatcher.message(filter::text("привет"), |context| async move {
        context.reply("И тебе привет!").await?;
        Ok(())
    });

    dispatcher.message(filter::contains("помощь"), |context| async move {
        context.answer("Напиши /start, чтобы начать.").await?;
        Ok(())
    });

    dispatcher.run_long_poll(group_id).await
}
