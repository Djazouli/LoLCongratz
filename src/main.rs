mod event_manager;

use crate::event_manager::EventManager;
use std::time::Duration;

#[tokio::main]
async fn main() {
    env_logger::init();

    let event_listener = EventManager::new();

    lol_game_client_api::start_listener(event_listener, Duration::from_secs(1));

    loop {
        tokio::time::sleep(Duration::from_secs(600)).await;
    }
}
