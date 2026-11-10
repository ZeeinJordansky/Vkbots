# Вызов методов VK API

Vkbots позволяет вызывать методы VK API без отдельной Rust-обёртки для каждого метода. Это сохраняет совместимость с новыми методами и параметрами API.

```rust
use serde_json::json;

let users: serde_json::Value = bot.api
    .call("users.get", &json!({
        "user_ids": "1"
    }))
    .await?;
```

Метод `call<T, P>` принимает имя метода, сериализуемые параметры и возвращает результат типа `T`.

## Типизированные обёртки

Для наиболее распространённых операций есть структуры запросов и ответов:

### users.get

```rust
use vkbots::methods::UsersGetParams;

let users = bot.api.users_get(&UsersGetParams {
    user_ids: vec![1, 2],
    fields: vec!["photo_100".into(), "domain".into()],
    name_case: None,
}).await?;
```

Результат — `Vec<User>`. Неизвестные поля ответа сохраняются в `User.extra`.

### messages.send

```rust
use vkbots::methods::MessagesSendParams;

let message_id = bot.api.messages_send(&MessagesSendParams {
    peer_id,
    random_id: 0,
    message: Some("Привет!".into()),
    reply_to: None,
    attachment: None,
    keyboard: None,
    payload: None,
    dont_parse_links: None,
    disable_mentions: None,
}).await?;
```

Для обычной отправки удобнее пользоваться `bot.send.answer()`, `bot.send.reply()` или builder.

### Методы загрузки фото

Доступны типизированные `photos_get_messages_upload_server()` и `photos_save_messages_photo()`. Полная последовательность загрузки файла автоматизирована в [разделе фотографий](uploads.md).

## Автоматические параметры

Библиотека добавляет:

- `access_token` — токен, переданный при создании клиента;
- `v` — версию VK API, установленную в builder.

Не передавайте эти параметры вручную. Примитивные массивы параметров кодируются через запятую, вложенные JSON-структуры — как JSON.

## Тип ответа

Выберите структуру, соответствующую JSON-ответу VK, либо используйте `serde_json::Value`, когда схема ответа может меняться.

Универсальный интерфейс не отменяет ограничений VK: доступность метода зависит от токена, прав, параметров сообщества и правил платформы. Дополнять типизированные обёртки можно по мере расширения API.
