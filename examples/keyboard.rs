use serde_json::json;
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

    dispatcher.command("/menu", |context| async move {
        let keyboard = json!({
            "one_time": false,
            "inline": true,
            "buttons": [[
                {
                    "action": {
                        "type": "text",
                        "label": "Поздороваться",
                        "payload": { "action": "hello" }
                    },
                    "color": "primary"
                }
            ]]
        });

        context.bot.send
            .to(context.message.as_ref().and_then(|m| m.peer_id).unwrap_or_default())
            .text("Нажмите кнопку.")
            .keyboard(&keyboard)?
            .send()
            .await?;
        Ok(())
    });

    dispatcher.message(filter::payload(json!({ "action": "hello" })), |context| async move {
        context.answer("Кнопка нажата!").await?;
        Ok(())
    });

    dispatcher.run_long_poll(group_id).await
}
