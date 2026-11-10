# Примеры

В каталоге `examples/` находятся самостоятельные программы:

- `long_poll.rs` — получение событий через Bots Long Poll;
- `callback.rs` — запуск Callback API сервера;
- `callback_nginx.rs` — Callback API за reverse proxy Nginx;
- `dispatcher.rs` — команды и фильтры;
- `fsm_registration.rs` — многошаговая регистрация с FSM;
- `keyboard.rs` — inline-клавиатура и обработка payload;
- `group_chat.rs` — ответы в групповой беседе.

Запуск Dispatcher:

```bash
VK_TOKEN=... VK_GROUP_ID=123456 cargo run --example dispatcher
```

Запуск FSM:

```bash
VK_TOKEN=... VK_GROUP_ID=123456 cargo run --example fsm_registration
```

Запуск Callback API за Nginx:

```bash
VK_TOKEN=... VK_GROUP_ID=123456 VK_CALLBACK_SECRET=... VK_CONFIRMATION_KEY=... cargo run --example callback_nginx
```

Пример конфигурации Nginx расположен в `config/nginx/vkbots.conf`.
