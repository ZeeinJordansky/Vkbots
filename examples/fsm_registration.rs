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

    dispatcher.command("/register", |context| async move {
        context.transition("await_name", &json!({})).await?;
        context.answer("Как вас зовут?").await?;
        Ok(())
    });

    dispatcher.message(filter::any_text(), |context| async move {
        match context.get_state().await?.as_deref() {
            Some("await_name") => {
                let name = context.message
                    .as_ref()
                    .and_then(|message| message.text.as_deref())
                    .unwrap_or("")
                    .trim()
                    .to_owned();

                context.transition("await_phone", &json!({ "name": name })).await?;
                context.answer("Укажите номер телефона.").await?;
            }
            Some("await_phone") => {
                let phone = context.message
                    .as_ref()
                    .and_then(|message| message.text.as_deref())
                    .unwrap_or("")
                    .trim()
                    .to_owned();

                let data: serde_json::Value = context.data().await?.unwrap_or_default();
                let name = data["name"].as_str().unwrap_or("Не указано");

                context.clear_state().await?;
                context.answer(format!("Регистрация завершена. Имя: {name}; телефон: {phone}")).await?;
            }
            _ => {}
        }
        Ok(())
    });

    dispatcher.run_long_poll(group_id).await
}
