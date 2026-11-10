# Обработка ошибок

Все основные методы возвращают `vkbots::Result<T>`.

Тип `Error` содержит варианты:

- `Http` — ошибка HTTP-клиента;
- `Json` — ошибка сериализации или разбора JSON;
- `VkApi` — ошибка, возвращённая VK API;
- `HttpStatus` — HTTP-ответ с ошибочным статусом;
- `InvalidParameter` — некорректный аргумент метода библиотеки;
- `Protocol` — неожиданный формат ответа;
- `Io` — ошибка сетевого сервера Callback API.

Пример:

```rust
match bot.send.answer(peer_id, "Привет").await {
    Ok(message_id) => println!("Отправлено: {message_id}"),
    Err(vkbots::Error::VkApi(error)) => {
        eprintln!("VK API {}: {}", error.error_code, error.error_msg);
    }
    Err(error) => eprintln!("Ошибка: {error}"),
}
```

Ошибка VK API содержит код, описание и параметры запроса, если VK включил их в ответ.
