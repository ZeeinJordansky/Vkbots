# Быстрый старт

Создайте файл `src/main.rs`:

```rust
use vkbots::VkBot;

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let bot = VkBot::new("ТОКЕН_СООБЩЕСТВА");

    bot.send.answer(123456, "Привет!").await?;
    Ok(())
}
```

Здесь `123456` — идентификатор диалога. Для личного диалога обычно используется ID пользователя, для бесед — peer_id беседы.

## Ответ на событие

```rust
use vkbots::{Message, VkBot};

async fn answer(bot: &VkBot, message: &Message) -> vkbots::Result<()> {
    bot.send.reply(message, "Получил ваше сообщение.").await?;
    Ok(())
}
```

У сообщения должны присутствовать `peer_id` и `id`. При отсутствии этих полей метод вернёт `Error::InvalidParameter`.
