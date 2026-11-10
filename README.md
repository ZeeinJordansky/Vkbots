# Vkbots

Асинхронная Rust-библиотека для разработки ботов ВКонтакте. Vkbots объединяет VK API, маршрутизацию команд, FSM, загрузку вложений, Callback API и Bots Long Poll.

[Полная документация](book/README.md) · [Исходный код](https://github.com/ZeeinJordansky/Vkbots) · [Примеры](examples/)

## Возможности

- Асинхронный клиент VK API на Tokio с переиспользованием HTTP-соединений.
- Универсальный вызов любого доступного токену метода через `bot.api.call()`.
- Типизированные обёртки для основных методов: `messages.send`, `users.get`, загрузки фото.
- Отправка сообщений через `send.answer()`, `send.reply()` и builder.
- Загрузка локальных фотографий через `attach_local_photo()`.
- Dispatcher команд и сообщений с фильтрами.
- FSM с хранилищем в памяти и интерфейсом для Redis/PostgreSQL.
- Callback API и Bots Long Poll.
- Ограничитель запросов и повторы временных ошибок VK.
- Unit- и integration-тесты, примеры Callback, клавиатур, бесед и FSM.

## Установка из GitHub

До публикации в crates.io добавьте зависимость из репозитория:

```toml
[dependencies]
vkbots = { git = "https://github.com/ZeeinJordansky/Vkbots" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
```

Стандартно включены features `callback` и `long-poll`. Для сборки только с нужным способом получения событий отключите стандартные features.

## Быстрый пример

```rust
use vkbots::VkBot;

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let bot = VkBot::new("ТОКЕН_СООБЩЕСТВА");
    bot.send.answer(123456, "Привет! Я работаю на Vkbots.").await?;
    Ok(())
}
```

## Команды и фильтры

```rust
use vkbots::{dispatcher::filter, VkBot};

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let bot = VkBot::new("ТОКЕН_СООБЩЕСТВА");
    let mut dispatcher = bot.dispatcher();

    dispatcher.command("/start", |context| async move {
        context.answer("Добро пожаловать!").await?;
        Ok(())
    });

    dispatcher.message(filter::text("привет"), |context| async move {
        context.reply("Привет!").await?;
        Ok(())
    });

    dispatcher.run_long_poll(123456).await
}
```

## FSM

```rust
dispatcher.command("/register", |context| async move {
    context.transition("await_name", &serde_json::json!({})).await?;
    context.answer("Как вас зовут?").await?;
    Ok(())
});
```

Подробный сценарий с несколькими шагами приведён в [примере FSM](examples/fsm_registration.rs).

## Отправка с фотографией

```rust
bot.send
    .to(peer_id)
    .text("Фото")
    .attach_local_photo("./img.png")
    .await?
    .send()
    .await?;
```

## Все методы VK API

Если для нужного метода ещё нет отдельной типизированной обёртки, можно вызвать его напрямую:

```rust
let response: serde_json::Value = bot.api
    .call("users.get", &serde_json::json!({ "user_ids": "1" }))
    .await?;
```

Библиотека автоматически добавляет `access_token` и `v`. Доступность метода определяется правами токена и ограничениями VK.

## Тесты и документация

```bash
cargo test --all-features
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features
```

Подробные страницы находятся в [book/](book/README.md). В каталоге `examples/` есть отдельные примеры Long Poll, Callback за Nginx, клавиатур, групповых бесед, Dispatcher и FSM.

## Лицензия

MIT. Полный текст — в [LICENSE](LICENSE).
