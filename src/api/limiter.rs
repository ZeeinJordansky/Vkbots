use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep, time::Instant};

/// Общий ограничитель частоты запросов для всех клонов API-клиента.
#[derive(Clone)]
pub(crate) struct RateLimiter {
    interval: Duration,
    next_slot: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    pub(crate) fn new(requests_per_second: u32) -> Self {
        let requests_per_second = requests_per_second.max(1);
        Self {
            interval: Duration::from_secs_f64(1.0 / f64::from(requests_per_second)),
            next_slot: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub(crate) async fn wait_turn(&self) {
        let delay = {
            let mut next = self.next_slot.lock().await;
            let now = Instant::now();
            let slot = (*next).max(now);
            *next = slot + self.interval;
            slot.saturating_duration_since(now)
        };

        if !delay.is_zero() {
            sleep(delay).await;
        }
    }
}
