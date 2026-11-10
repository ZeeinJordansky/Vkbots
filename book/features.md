# Cargo features

| Feature | Назначение |
| --- | --- |
| `callback` | HTTP-сервер Callback API на Axum |
| `long-poll` | Клиент Bots Long Poll |

Обе возможности включены по умолчанию.

Только API-клиент и отправка сообщений:

```toml
vkbots = { version = "0.1", default-features = false }
```

Только Long Poll:

```toml
vkbots = { version = "0.1", default-features = false, features = ["long-poll"] }
```

Только Callback API:

```toml
vkbots = { version = "0.1", default-features = false, features = ["callback"] }
```
