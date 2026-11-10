# Отправка сообщений

## Простая отправка

```rust
bot.send.answer(peer_id, "Привет!").await?;
```

Метод возвращает идентификатор сообщения, полученный от VK.

## Ответ на сообщение

```rust
bot.send.reply(&message, "Это ответ.").await?;
```

Библиотека возьмёт `peer_id` и `id` из модели сообщения и передаст `reply_to` в VK API.

## Builder

Для дополнительных параметров используйте `to()`:

```rust
let message_id = bot.send
    .to(peer_id)
    .text("Выберите действие")
    .keyboard(&keyboard)?
    .disable_mentions(true)
    .send()
    .await?;
```

Доступные методы builder:

- `text(value)` — текст сообщения.
- `reply_to(id)` — ID сообщения для ответа.
- `attachment(value)` — строка вложения VK.
- `attachments(values)` — несколько вложений через запятую.
- `attach_local_photo(path)` — загрузить локальное фото, сохранить в VK и добавить в сообщение.
- `keyboard(value)` — клавиатура в формате VK JSON.
- `payload(value)` — payload сообщения.
- `random_id(value)` — собственный идентификатор запроса.
- `dont_parse_links(value)` — отключение распознавания ссылок.
- `disable_mentions(value)` — отключение упоминаний.
- `parameter(name, value)` — дополнительный параметр `messages.send`.

Builder проверяет, что указан хотя бы один из элементов: текст, вложение, стикер или шаблон. Полный список допустимых параметров определяется документацией VK API для выбранной версии.
