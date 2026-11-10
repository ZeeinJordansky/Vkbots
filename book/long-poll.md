# Bots Long Poll

Bots Long Poll подходит для приложений, которые самостоятельно подключаются к серверу обновлений VK.

```rust
use vkbots::{long_poll::LongPoll, VkBot};

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let bot = VkBot::new("ТОКЕН_СООБЩЕСТВА");
    let mut poll = LongPoll::connect(bot.clone(), 123456).await?;

    loop {
        let event = poll.next_event().await?;
        if let Some(message) = event.message() {
            println!("{}", message.text.as_deref().unwrap_or(""));
            bot.send.reply(&message, "Сообщение получено.").await?;
        }
    }
}
```

## Параметры

Метод `wait(seconds)` задаёт время ожидания ответа Long Poll. Допустимый диапазон библиотеки — от 1 до 90 секунд.

## Переподключение

Клиент самостоятельно обрабатывает служебные ответы Long Poll, обновляет временную метку и заново получает параметры подключения при ошибках ключа или временной метки.

События поступают по одному через `next_event()`. Если VK вернул несколько обновлений за один запрос, они сохраняются во внутренней очереди.
