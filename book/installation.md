# Установка

Vkbots публикуется как crate `vkbots`. Пока пакет не опубликован в crates.io, подключайте его напрямую из GitHub.

Добавьте зависимости в `Cargo.toml`:

```toml
[dependencies]
vkbots = { git = "https://github.com/ZeeinJordansky/Vkbots" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
```

По умолчанию активны компоненты Callback API и Bots Long Poll. Если один из них не нужен, отключите стандартные features:

```toml
[dependencies]
vkbots = {
    git = "https://github.com/ZeeinJordansky/Vkbots",
    default-features = false,
    features = ["long-poll"]
}
```

Для Callback API используйте feature `callback`, для Bots Long Poll — `long-poll`.

## Проверка установки

```bash
cargo check
cargo test --all-features
```

Токен должен принадлежать сообществу и иметь права, необходимые для вызываемых методов. Vkbots не получает токены автоматически.

После публикации в crates.io зависимость можно будет указать как `vkbots = "0.2"`.
