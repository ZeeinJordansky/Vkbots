# Callback API

Callback API отправляет HTTP POST-запросы на адрес приложения при возникновении событий сообщества. Сервер должен быть доступен VK по HTTP(S).

## Запуск

```rust
use std::net::SocketAddr;
use vkbots::callback::{CallbackConfig, CallbackServer};

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let config = CallbackConfig::new(123456)
        .secret("СЕКРЕТНЫЙ_КЛЮЧ")
        .confirmation_key("СТРОКА_ПОДТВЕРЖДЕНИЯ");

    CallbackServer::new(config)
        .on_event(|event| async move {
            println!("Новое событие: {}", event.event_type);
        })
        .serve("0.0.0.0:3000".parse::<SocketAddr>().unwrap())
        .await
}
```

В настройках Callback API сообщества укажите публичный адрес сервера и тот же секретный ключ.

## Встраивание в Axum

Вместо запуска отдельного сервера можно встроить готовый маршрутизатор:

```rust
let callback = CallbackServer::new(config).on_event(|event| async move {
    println!("{}", event.event_type);
});

let app = axum::Router::new().nest("/vk", callback.router());
```

Обработчик будет доступен по маршруту `/vk/`.

## Ошибки обработчика

`on_event()` подходит для обработчиков, которые самостоятельно обрабатывают ошибки. Для возврата ошибки используйте `on_event_result()`:

```rust
CallbackServer::new(config)
    .on_event_result(|event| async move {
        process_event(event).await
    });
```

Если обработчик возвращает ошибку, сервер отвечает HTTP 500. VK сможет повторно доставить событие. Учитывайте повторную доставку: сохраняйте обработанные `event_id` и делайте обработчики идемпотентными.

## Проверки

Vkbots проверяет идентификатор сообщества и секретный ключ, если он задан. Для события `confirmation` возвращается строка подтверждения. Для принятого события после завершения обработчика возвращается `ok`.
