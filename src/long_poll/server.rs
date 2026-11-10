use std::collections::VecDeque;

use serde::Deserialize;
use serde_json::Value;

use crate::{client::VkBot, error::{Error, Result}, Event};

/// Клиент группового Long Poll.
pub struct LongPoll {
    bot: VkBot,
    group_id: i64,
    server: LongPollServerState,
    queue: VecDeque<Event>,
    wait: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct LongPollServerState {
    key: String,
    server: String,
    ts: String,
}

#[derive(Deserialize)]
struct LongPollResponse {
    #[serde(default)]
    ts: Option<String>,
    #[serde(default)]
    updates: Vec<Value>,
    #[serde(default)]
    failed: Option<i64>,
}

impl LongPoll {
    /// Подключается к Long Poll сообщества.
    pub async fn connect(bot: VkBot, group_id: i64) -> Result<Self> {
        if group_id <= 0 {
            return Err(Error::InvalidParameter(
                "group_id должен быть положительным идентификатором сообщества".to_owned(),
            ));
        }

        let server = bot.api.groups_get_long_poll_server(group_id).await?;
        Ok(Self {
            bot,
            group_id,
            server: LongPollServerState {
                key: server.key,
                server: server.server,
                ts: server.ts,
            },
            queue: VecDeque::new(),
            wait: 25,
        })
    }

    /// Устанавливает время ожидания на стороне VK (от 1 до 90 секунд).
    pub fn wait(mut self, seconds: u8) -> Self {
        self.wait = seconds.clamp(1, 90);
        self
    }

    /// Возвращает следующее событие. При отсутствии новых событий продолжает ждать.
    pub async fn next_event(&mut self) -> Result<Event> {
        loop {
            if let Some(event) = self.queue.pop_front() {
                return Ok(event);
            }

            let wait = self.wait.to_string();
            let response = self
                .bot
                .api
                .http_client()
                .get(&self.server.server)
                .query(&[
                    ("act", "a_check"),
                    ("key", self.server.key.as_str()),
                    ("ts", self.server.ts.as_str()),
                    ("wait", wait.as_str()),
                    ("mode", "2"),
                ])
                .send()
                .await?
                .error_for_status()?
                .json::<LongPollResponse>()
                .await?;

            match response.failed {
                Some(1) => {
                    if let Some(ts) = response.ts {
                        self.server.ts = ts;
                    }
                    continue;
                }
                Some(2 | 3) => {
                    self.refresh().await?;
                    continue;
                }
                Some(code) => {
                    return Err(Error::Protocol(format!(
                        "Long Poll вернул неподдерживаемый код failed={code}"
                    )));
                }
                None => {}
            }

            if let Some(ts) = response.ts {
                self.server.ts = ts;
            }

            self.queue.extend(
                response
                    .updates
                    .into_iter()
                    .map(|update| Event::from_long_poll(self.group_id, update)),
            );
        }
    }

    async fn refresh(&mut self) -> Result<()> {
        let server = self.bot.api.groups_get_long_poll_server(self.group_id).await?;
        self.server = LongPollServerState {
            key: server.key,
            server: server.server,
            ts: server.ts,
        };
        Ok(())
    }
}
