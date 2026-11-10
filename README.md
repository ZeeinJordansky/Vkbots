# Vkbots

Асинхронная Rust-библиотека для создания ботов ВКонтакте. Она объединяет работу с VK API, отправку сообщений, Callback API и Bots Long Poll в одном интерфейсе.

[Документация](book/README.md) · [Исходный код](https://github.com/ZeeinJordansky/Vkbots)

## Возможности

- Асинхронный клиент VK API на Tokio.
- Универсальный вызов любого доступного токену метода VK API через `bot.api.call()`.
- Короткие методы отправки: `bot.send.answer()` и `bot.send.reply()`.
- Builder отправки с поддержкой вложений, клавиатуры и дополнительных параметров.
- Получение событий через Callback API или Bots Long Poll.
- Общая модель события для обоих способов получения обновлений.
- Типизированные ошибки VK API и HTTP.
- Настраиваемые версия API и HTTP-тайм-аут.
- Скрытие токена в отладочном выводе.
- Независимое подключение компонентов через Cargo features.

## Установка

Добавьте библиотеку в `Cargo.toml`:

```toml
[dependencies]
vkbots = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
```

По умолчанию включены компоненты `callback` и `long-poll`. Для отключения стандартных компонентов используйте `default-features = false`.

## Первый бот

```rust
use vkbots::VkBot;

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let bot = VkBot::new("ТОКЕН_СООБЩЕСТВА");
    let message_id = bot.send.answer(123456, "Привет! Я работаю на Vkbots.").await?;
    println!("Сообщение отправлено: {message_id}");
    Ok(())
}
```

## Ответ на сообщение

```rust
use vkbots::{Message, VkBot};

async fn on_message(bot: &VkBot, message: &Message) -> vkbots::Result<()> {
    bot.send.reply(message, "Это ответ на ваше сообщение.").await?;
    Ok(())
}
```

## Любой метод VK API

```rust
use serde_json::json;
use vkbots::VkBot;

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let bot = VkBot::new("ТОКЕН_СООБЩЕСТВА");
    let users: serde_json::Value = bot.api
        .call("users.get", &json!({ "user_ids": "1" }))
        .await?;
    println!("{users:#}");
    Ok(())
}
```

Библиотека добавляет `access_token` и `v` автоматически. Доступность конкретного метода определяется VK, типом токена, правами сообщества и ограничениями платформы. Универсальный интерфейс позволяет вызывать методы, для которых ещё нет отдельной обёртки.

## Выбор способа получения событий

| Способ | Когда использовать |
| --- | --- |
| Bots Long Poll | Для отдельного процесса бота, который самостоятельно получает обновления |
| Callback API | Для приложения с публичным HTTP(S)-адресом, принимающего запросы VK |

Подробные руководства, примеры и описание API находятся в каталоге [book](book/README.md).

## Лицензия

Vkbots распространяется по лицензии MIT. Полный текст находится в файле [LICENSE](LICENSE).
