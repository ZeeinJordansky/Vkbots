use std::{future::Future, sync::Arc};

use futures_util::future::BoxFuture;

use crate::{
    error::Result,
    fsm::{SessionKey, StateMachine, StateStorage},
    Event, Message, VkBot,
};

#[cfg(feature = "callback")]
use crate::callback::{CallbackConfig, CallbackServer};
#[cfg(feature = "long-poll")]
use crate::long_poll::LongPoll;

use super::{Context, Filter};

type Handler = Arc<dyn Fn(Context) -> BoxFuture<'static, Result<()>> + Send + Sync>;

#[derive(Clone)]
enum RouteKind {
    Command(String),
    Message(Filter),
    State(String),
}

#[derive(Clone)]
struct Route {
    kind: RouteKind,
    handler: Handler,
}

/// Маршрутизатор событий бота.
#[derive(Clone)]
pub struct Dispatcher {
    bot: VkBot,
    state: StateMachine,
    routes: Vec<Route>,
}

impl Dispatcher {
    /// Создаёт диспетчер с FSM-хранилищем в памяти.
    pub fn new(bot: VkBot) -> Self {
        Self::with_fsm(bot, StateMachine::new())
    }

    /// Создаёт диспетчер с заранее настроенным FSM.
    pub fn with_fsm(bot: VkBot, state: StateMachine) -> Self {
        Self {
            bot,
            state,
            routes: Vec::new(),
        }
    }

    /// Создаёт диспетчер с пользовательским хранилищем FSM.
    pub fn with_storage(bot: VkBot, storage: Arc<dyn StateStorage>) -> Self {
        Self::with_fsm(bot, StateMachine::with_storage(storage))
    }

    /// Регистрирует команду, например /start или start.
    pub fn command<F, Fut>(&mut self, command: impl Into<String>, handler: F) -> &mut Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let command = normalize_command(&command.into());
        self.routes.push(Route {
            kind: RouteKind::Command(command),
            handler: Arc::new(move |context| Box::pin(handler(context))),
        });
        self
    }

    /// Регистрирует обработчик сообщения с фильтром.
    pub fn message<F, Fut>(&mut self, filter: Filter, handler: F) -> &mut Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.routes.push(Route {
            kind: RouteKind::Message(filter),
            handler: Arc::new(move |context| Box::pin(handler(context))),
        });
        self
    }

    /// Регистрирует обработчик, который вызывается при активном состоянии FSM.
    pub fn on_state<F, Fut>(&mut self, state: impl Into<String>, handler: F) -> &mut Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.routes.push(Route {
            kind: RouteKind::State(state.into()),
            handler: Arc::new(move |context| Box::pin(handler(context))),
        });
        self
    }

    /// Передаёт событие первому совпавшему обработчику.
    ///
    /// Возвращает true, если событие было обработано.
    pub async fn dispatch(&self, event: Event) -> Result<bool> {
        let Some(message) = event.message() else {
            return Ok(false);
        };
        let session_key = SessionKey::from_message(&message);
        let current_state = if self
            .routes
            .iter()
            .any(|route| matches!(&route.kind, RouteKind::State(_)))
        {
            match session_key.as_ref() {
                Some(key) => self.state.state(key).await?,
                None => None,
            }
        } else {
            None
        };

        let context = Context::new(self.bot.clone(), event, self.state.clone());

        for route in &self.routes {
            let matched = match &route.kind {
                RouteKind::Command(command) => message_command(&message)
                    .as_deref()
                    .is_some_and(|received| received == command),
                RouteKind::Message(filter) => filter.check(&message),
                RouteKind::State(state) => current_state.as_deref() == Some(state.as_str()),
            };

            if matched {
                (route.handler)(context).await?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Запускает цикл получения обновлений через Bots Long Poll.
    #[cfg(feature = "long-poll")]
    pub async fn run_long_poll(&self, group_id: i64) -> Result<()> {
        let mut poll = LongPoll::connect(self.bot.clone(), group_id).await?;
        loop {
            let event = poll.next_event().await?;
            self.dispatch(event).await?;
        }
    }

    /// Создаёт Callback API сервер, который передаёт события этому диспетчеру.
    #[cfg(feature = "callback")]
    pub fn callback_server(&self, config: CallbackConfig) -> CallbackServer {
        let dispatcher = self.clone();
        CallbackServer::new(config).on_event_result(move |event| {
            let dispatcher = dispatcher.clone();
            async move {
                dispatcher.dispatch(event).await.map(|_| ())
            }
        })
    }

    /// Возвращает FSM диспетчера для ручной работы с состояниями.
    pub fn state(&self) -> StateMachine {
        self.state.clone()
    }
}

fn normalize_command(value: &str) -> String {
    value
        .trim()
        .trim_start_matches('/')
        .split('@')
        .next()
        .unwrap_or("")
        .to_lowercase()
}

fn message_command(message: &Message) -> Option<String> {
    let first = message.text.as_deref()?.trim().split_whitespace().next()?;
    if !first.starts_with('/') {
        return None;
    }
    Some(normalize_command(first))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        dispatcher::{filter, Dispatcher},
        fsm::SessionKey,
        Event, VkBot,
    };

    fn message_event(text: &str) -> Event {
        serde_json::from_value(json!({
            "type": "message_new",
            "group_id": 1,
            "object": {
                "message": {
                    "id": 10,
                    "peer_id": 2000000001,
                    "from_id": 42,
                    "text": text
                }
            }
        }))
        .unwrap()
    }

    #[tokio::test]
    async fn command_route_matches_command_with_bot_mention() {
        let bot = VkBot::new("test-token");
        let mut dispatcher = Dispatcher::new(bot);
        dispatcher.command("/start", |_context| async { Ok(()) });

        assert!(dispatcher.dispatch(message_event("/start@my_bot")).await.unwrap());
        assert!(!dispatcher.dispatch(message_event("/help")).await.unwrap());
    }

    #[tokio::test]
    async fn message_filter_matches_without_case_sensitivity() {
        let bot = VkBot::new("test-token");
        let mut dispatcher = Dispatcher::new(bot);
        dispatcher.message(filter::text("Привет"), |_context| async { Ok(()) });

        assert!(dispatcher.dispatch(message_event("  ПРИВЕТ ")).await.unwrap());
        assert!(!dispatcher.dispatch(message_event("пока")).await.unwrap());
    }

    #[tokio::test]
    async fn state_route_matches_current_fsm_step() {
        let bot = VkBot::new("test-token");
        let mut dispatcher = Dispatcher::new(bot);
        dispatcher.on_state("await_name", |_context| async { Ok(()) });

        dispatcher
            .state()
            .set_state(&SessionKey::new(2000000001, 42), "await_name")
            .await
            .unwrap();

        assert!(dispatcher.dispatch(message_event("Роман")).await.unwrap());
    }
}
