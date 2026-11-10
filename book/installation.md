# Установка

Vkbots публикуется как crate `vkbots`. Для использования требуется Rust toolchain и Cargo.

Добавьте зависимости в `Cargo.toml`:

```toml
[dependencies]
vkbots = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
```

По умолчанию активны компоненты Callback API и Bots Long Poll. Если один из них не нужен, отключите стандартные features:

```toml
[dependencies]
vkbots = { version = "0.1", default-features = false, features = ["long-poll"] }
```

Для Callback API используйте feature `callback`, для Bots Long Poll — `long-poll`.

## Проверка установки

```bash
cargo check
```

Токен должен принадлежать сообществу и иметь права, необходимые для вызываемых методов. Vkbots не получает токены автоматически.
