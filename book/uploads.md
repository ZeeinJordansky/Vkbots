# Загрузка фотографий

Для загрузки локальной фотографии библиотека выполняет полный цикл VK:

1. Запрашивает адрес сервера через `photos.getMessagesUploadServer`.
2. Отправляет файл на полученный `upload_url` multipart-запросом.
3. Сохраняет результат через `photos.saveMessagesPhoto`.
4. Преобразует сохранённую фотографию в строку вложения VK и добавляет её в сообщение.

## Пример

```rust
bot.send
    .to(peer_id)
    .text("Фотография")
    .attach_local_photo("./img.png")
    .await?
    .send()
    .await?;
```

Файл читается асинхронно. Метод возвращает ошибку, если файл недоступен, сервер загрузки ответил ошибкой или VK не вернул сохранённое вложение.

## Отдельная загрузка

Можно загрузить фотографию отдельно:

```rust
let photo = bot.api.upload_message_photo(peer_id, "./img.png").await?;
let attachment = photo.attachment();
bot.send.to(peer_id).attachment(attachment).send().await?;
```

Для нестандартных сценариев доступны типизированные методы `photos_get_messages_upload_server` и `photos_save_messages_photo`.
