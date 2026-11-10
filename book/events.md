# События и модели

## Event

Общая структура события Callback API и Long Poll содержит:

- `event_type` — тип события;
- `group_id` — идентификатор сообщества;
- `object` — объект события в формате JSON;
- `event_id` — идентификатор события, если он предоставлен VK;
- `secret` — секретное значение Callback API, если оно присутствует;
- `v` — версия события.

Неизвестные поля объекта не удаляются: они доступны через `object`.

## Получение сообщения

```rust
if let Some(message) = event.message() {
    let text = message.text.as_deref().unwrap_or("");
    println!("{text}");
}
```

## Message

Модель сообщения содержит основные поля: `id`, `date`, `peer_id`, `from_id`, `text`, `conversation_message_id`, `payload` и `attachments`.

Дополнительные поля сохраняются в `extra`, поэтому при появлении новых полей VK данные не теряются при десериализации.
