//! Асинхронная библиотека для создания ботов ВКонтакте.
//!
//! Vkbots объединяет вызовы VK API, маршрутизацию событий, FSM,
//! отправку сообщений и получение событий через Callback API или Bots Long Poll.

pub mod api;
#[cfg(feature = "callback")]
pub mod callback;
pub mod client;
pub mod dispatcher;
pub mod error;
pub mod fsm;
#[cfg(feature = "long-poll")]
pub mod long_poll;
pub mod methods;
pub mod models;
pub mod send;

pub use client::{VkBot, VkBotBuilder};
pub use error::{Error, Result, VkApiError};
pub use models::{Event, Message};

#[cfg(test)]
mod tests;
