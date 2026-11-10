use std::env;

use vkbots::{long_poll::LongPoll, VkBot};

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let token = env::var("VK_TOKEN").expect("Укажите VK_TOKEN");
    let group_id: i64 = env::var("VK_GROUP_ID")
        .expect("Укажите VK_GROUP_ID")
        .parse()
        .expect("VK_GROUP_ID должен быть числом");

    let bot = VkBot::new(token);
    let mut long_poll = LongPoll::connect(bot.clone(), group_id).await?;

    loop {
        let event = long_poll.next_event().await?;
        if let Some(message) = event.message() {
            let text = message.text.as_deref().unwrap_or("");
            println!("Новое сообщение: {text}");
            bot.send.reply(&message, "Сообщение получено.").await?;
        }
    }
}
