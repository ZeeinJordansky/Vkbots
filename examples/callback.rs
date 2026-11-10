use std::{env, net::SocketAddr};

use vkbots::callback::{CallbackConfig, CallbackServer};

#[tokio::main]
async fn main() -> vkbots::Result<()> {
    let group_id: i64 = env::var("VK_GROUP_ID")
        .expect("Укажите VK_GROUP_ID")
        .parse()
        .expect("VK_GROUP_ID должен быть числом");
    let secret = env::var("VK_CALLBACK_SECRET").ok();
    let confirmation = env::var("VK_CONFIRMATION_KEY").ok();

    let mut config = CallbackConfig::new(group_id);
    if let Some(secret) = secret {
        config = config.secret(secret);
    }
    if let Some(confirmation) = confirmation {
        config = config.confirmation_key(confirmation);
    }

    let address: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    CallbackServer::new(config)
        .on_event(|event| async move {
            if let Some(message) = event.message() {
                println!(
                    "Событие {}: {}",
                    event.event_type,
                    message.text.as_deref().unwrap_or("")
                );
            }
        })
        .serve(address)
        .await
}
