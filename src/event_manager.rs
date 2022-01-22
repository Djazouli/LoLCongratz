use lazy_static::lazy_static;
use lol_game_client_api::api::{GameClient, QueryError};
use lol_game_client_api::async_trait::async_trait;
use lol_game_client_api::event_listener::EventListener;
use lol_game_client_api::model::{Ace, ChampionKill, GameStart, Multikill, Player, Team};
use rand::seq::SliceRandom;
use rodio::OutputStreamHandle;
use std::collections::HashMap;
use std::io::BufReader;
use thiserror::Error;

// Load the mapping between Summoner name and sound file
lazy_static! {
    static ref MAPPING: HashMap<String, Vec<String>> =
        serde_json::from_reader(std::fs::File::open("sounds/mapping.json").unwrap()).unwrap();
}

pub struct EventManager {
    game_client: GameClient, // Is used to query the team if needed, or other info on the fly
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            game_client: GameClient::new(),
        }
    }

    /// Get the team of the current active player
    async fn get_current_team(&self) -> Result<Team, EventManagerError> {
        let active_player = self.game_client.active_player().await?.summoner_name;

        self.get_player(active_player.as_str())
            .await
            .map(|player| player.team)
    }

    /// Get player struct, given the player name.
    async fn get_player(&self, player_name: impl AsRef<str>) -> Result<Player, EventManagerError> {
        let players = self.game_client.player_list().await?;

        for player in players {
            if player.summoner_name.as_str() == player_name.as_ref() {
                return Ok(player);
            }
        }

        Err(EventManagerError::PlayerNotFound)
    }
}

#[derive(Error, Debug)]
pub enum EventManagerError {
    #[error("Game client API error: {}", _0)]
    GameClientApi(#[from] QueryError),
    #[error("Failed to find active player in player list")]
    PlayerNotFound,
}

#[async_trait]
impl EventListener for EventManager {
    type Error = EventManagerError;

    async fn on_game_start(&mut self, _event: GameStart) -> Result<(), Self::Error> {
        play_sound(
            MAPPING
                .get("game_start")
                .unwrap()
                .choose(&mut rand::thread_rng()),
        );
        Ok(())
    }

    async fn on_champion_kill(&mut self, event: ChampionKill) -> Result<(), Self::Error> {
        let player = self.get_player(event.killer_name).await?;
        if player.team != self.get_current_team().await? {
            // Only play sound for our team
            return Ok(());
        }

        play_congratz(&player);
        Ok(())
    }

    async fn on_multi_kill(&mut self, event: Multikill) -> Result<(), Self::Error> {
        let player = self.get_player(event.killer_name).await?;
        if player.team != self.get_current_team().await? {
            // Only play sound for our team
            return Ok(());
        }

        play_congratz(&player);
        Ok(())
    }

    async fn on_ace(&mut self, event: Ace) -> Result<(), Self::Error> {
        if self.get_current_team().await? == event.acing_team {
            log::info!("We aced !");
            play_sound(MAPPING.get("ace").unwrap().choose(&mut rand::thread_rng()));
        } else {
            log::info!("We got aced :(");
            play_sound(MAPPING.get("aced").unwrap().choose(&mut rand::thread_rng()));
        }

        Ok(())
    }
}

fn play_congratz(player: &Player) {
    play_sound(get_sound_path(player));
}

/// Given a player, pick a random sound to play in the mapping
fn get_sound_path(player: &Player) -> Option<&String> {
    MAPPING
        .get(&player.summoner_name)
        .unwrap_or_else(|| MAPPING.get("default").unwrap())
        .choose(&mut rand::thread_rng())
}

#[derive(Error, Debug)]
pub enum SoundPlayerError {
    #[error("IoError: {}", _0)]
    IoError(#[from] std::io::Error),
}

pub fn play_sound(filename: Option<impl Into<String>>) {
    if let Some(filename) = filename {
        let full_filename = format!("sounds/{}", filename.into());
        let full_filename_copy = full_filename.clone();
        // Spawn separate threads, otherwise it will pause the whole current thread

        // Play the sound in vb cable
        std::thread::spawn(move || {
            let vb_cable = crate::get_vb_cable();
            let (_stream, vb_handle) =
                rodio::OutputStream::try_from_device(&vb_cable.unwrap()).unwrap();
            _play_sound(full_filename, vb_handle)
        });

        // play the sound in your output
        std::thread::spawn(move || {
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            _play_sound(full_filename_copy, handle)
        });
    }
}

/// Unwrap-heavy function, needs to be called in another thread.
/// Given a full path to an audio file, play it.
pub fn _play_sound(filename: String, handle: OutputStreamHandle) {
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(filename).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}
