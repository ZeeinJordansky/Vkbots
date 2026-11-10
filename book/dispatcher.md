# Dispatcher и фильтры

Dispatcher избавляет от ручного разбора каждой строки и позволяет регистрировать обработчики отдельно.

## Первый обработчик

```rust
use vkbots::{dispatcher::filter, VkBot};

let bot = VkBot::new("ТОКЕН");
let mut dispatcher = bot.dispatcher();

dispatcher.command("/start", |context| async move {
    context.answer("Бот запущен.").await?;
    Ok(())
});

dispatcher.message(filter::text("привет"), |context| async move {
    context.reply("Привет!").await?;
    Ok(())
});

dispatcher.run_long_poll(group_id).await?;
```

## Команды

`command("/start", handler)` принимает имя команды с косой чертой или без неё. Входящая команда `/start@community_name` также распознаётся. Аргументы после команды доступны через `context.command_args()`.

## Фильтры

- `filter::text("привет")` — точное совпадение без учёта регистра и внешних пробелов.
- `filter::contains("слово")` — текст содержит фрагмент.
- `filter::starts_with("начало")` — текст начинается с фрагмента.
- `filter::any_text()` — любое непустое текстовое сообщение.
- `filter::payload(json!({...}))` — совпадение payload кнопки.

Если совпало несколько условий, выполняется первый зарегистрированный обработчик. Чтобы избежать дублирующих ответов, порядок регистрации имеет значение.

## Контекст

Обработчик получает `Context` с полями `bot`, `event`, `message`. Методы `answer()` и `reply()` отправляют сообщения в текущий диалог, а `command_args()` возвращает аргументы команды.

## Callback API

`dispatcher.callback_server(config)` создаёт CallbackServer, который передаёт события в те же обработчики. Запуск сервера описан в [разделе Callback API](callback.md).
