# FSM — конечный автомат состояний

FSM хранит текущий шаг пользовательского сценария и связанные с ним данные. Сессия отделяется по паре `peer_id + user_id`, поэтому один пользователь может иметь разные сценарии в личных сообщениях и разных беседах.

## Пример сценария

```rust
use serde_json::json;
use vkbots::{dispatcher::filter, VkBot};

let bot = VkBot::new("ТОКЕН");
let mut dispatcher = bot.dispatcher();

dispatcher.command("/register", |ctx| async move {
    ctx.transition("await_name", &json!({})).await?;
    ctx.answer("Как вас зовут?").await?;
    Ok(())
});

dispatcher.message(filter::any_text(), |ctx| async move {
    match ctx.get_state().await?.as_deref() {
        Some("await_name") => {
            let name = ctx.message.as_ref()
                .and_then(|message| message.text.as_deref())
                .unwrap_or("")
                .trim()
                .to_owned();

            ctx.transition("await_phone", &json!({ "name": name })).await?;
            ctx.answer("Укажите номер телефона.").await?;
        }
        Some("await_phone") => {
            let phone = ctx.message.as_ref()
                .and_then(|message| message.text.as_deref())
                .unwrap_or("")
                .trim();

            let data: serde_json::Value = ctx.data().await?.unwrap_or_default();
            let name = data["name"].as_str().unwrap_or("Не указано");

            ctx.clear_state().await?;
            ctx.answer(format!("Имя: {name}; телефон: {phone}")).await?;
        }
        _ => {}
    }
    Ok(())
});
```

## Методы контекста

- `get_state()` — получить имя текущего состояния.
- `set_state(name)` — изменить состояние без удаления пользовательских данных.
- `set_data(value)` — сохранить сериализуемые данные.
- `data::<T>()` — прочитать данные с десериализацией в тип `T`.
- `transition(name, data)` — сохранить новый шаг и данные одной операцией.
- `clear_state()` — удалить сессию.

## Хранилище

По умолчанию используется `MemoryStorage`. Оно хранит сессии в памяти процесса и очищается после перезапуска.

Для Redis, PostgreSQL или другого постоянного хранилища реализуйте асинхронный trait `StateStorage` с методами `load`, `save` и `delete`, затем передайте его в `Dispatcher::with_storage`.

Хранилище само по себе не делает бизнес-операции атомарными. Для параллельной обработки сообщений одного пользователя постоянная реализация должна учитывать конкурентные обновления сессии.
