mod event_manager;

#[cfg(feature = "voice_proxy")]
mod vb_cable_proxy;

use crate::event_manager::{play_sound, EventManager};
use std::time::Duration;

#[tokio::main]
async fn main() {
    env_logger::init();

    let event_listener = EventManager::new();

    lol_game_client_api::start_listener(event_listener, Duration::from_secs(1));

    #[cfg(feature = "voice_proxy")]
    vb_cable_proxy::start();

    loop {
        tokio::time::sleep(Duration::from_secs(600)).await;
    }
}
