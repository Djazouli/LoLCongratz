mod event_manager;

#[cfg(feature = "voice_proxy")]
mod vb_cable_proxy;

use crate::event_manager::{play_sound, EventManager};
use log::LevelFilter;
use rodio::cpal::traits::HostTrait;
use rodio::{Device, DeviceTrait};
use std::time::Duration;

/// Get VB-cable device if it can be found.
pub fn get_vb_cable() -> Option<Device> {
    let host = rodio::cpal::default_host();
    let devices = host.output_devices().unwrap();

    for device in devices {
        if let Ok(name) = device.name() {
            if name.as_str() == "VB-Cable" // OSx
                || name.as_str() == "CABLE Input (VB-Audio Virtual Cable)"
            // Windows
            {
                return Some(device);
            }
        }
    }

    None
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();

    let event_listener = EventManager::new();

    lol_game_client_api::start_listener(event_listener, Duration::from_secs(1));

    #[cfg(feature = "voice_proxy")]
    vb_cable_proxy::start();

    loop {
        tokio::time::sleep(Duration::from_secs(600)).await;
    }
}
