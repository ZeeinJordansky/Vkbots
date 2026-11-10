# Примеры

В каталоге `examples/` находятся самостоятельные примеры:

- `long_poll.rs` — получение событий через Bots Long Poll;
- `callback.rs` — запуск Callback API сервера.

Запуск Long Poll:

```bash
VK_TOKEN=... VK_GROUP_ID=123456 cargo run --example long_poll
```

Запуск Callback API:

```bash
VK_GROUP_ID=123456 VK_CALLBACK_SECRET=... VK_CONFIRMATION_KEY=... cargo run --example callback
```

В Windows значения переменных окружения задаются средствами используемой оболочки. Для production-развёртывания используйте менеджер секретов платформы или защищённую конфигурацию сервера.
