use std::collections::HashMap;

pub use rspotify::{AuthCodeSpotify, Credentials, OAuth, model::{CurrentlyPlayingType::Unknown, PlayableItem, track}, prelude::*, scopes};
use serde::{Serialize, Deserialize};

pub const DATA_PATH: &str = "data/";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserData {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Song {
    pub name: Option<String>,
    pub artist: Option<String>,
    pub duration_ms: Option<u64>,
    pub user_data: Vec<UserData>,
}

impl Song {
    pub fn print_preview(&self, prefix: &str){
        println!("{}", prefix);
        println!("  name: {}", self.name.as_deref().unwrap_or("unknown"));
        println!("  artist: {}",self.artist.as_deref().unwrap_or("unknown"));

        for key_value in &self.user_data {
            println!(" {} : {}", key_value.key, key_value.value);
        }
    }
}

pub async fn authorize() -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    spotify
}

pub fn get_song(item: PlayableItem) -> Option<(Song, String)>{
     match item {
        PlayableItem::Unknown(object) => {
            let id: String = object["id"].as_str()?.to_string();

            let song = Song {
                name: object["name"].as_str().map(String::from),
                artist: object["artists"][0]["name"].as_str().map(String::from),
                duration_ms: object["duration_ms"].as_u64(),
                user_data: Vec::new()
            };
            Some((song, id))
        }
        unhandled => {
            println!("Unimplemented playback item type {:#?}", unhandled);
            None
        }
    }
}

pub fn load_playlist(path: &str) -> Option<HashMap<String, Song>> {
    let file = std::fs::read_to_string(path).ok()?;
    let map: HashMap<String, Song> = serde_json::from_str(&file).ok()?;
    Some(map)
}

pub fn save_playlist(path: &str, songs: &HashMap<String, Song>){
    std::fs::write(
        path,
        serde_json::to_string_pretty(songs).expect("Could not serialize to .json")
    ).expect("Could not write to file");
}

pub fn make_playlist_path(name: &str) -> String{
    format!("{}{}.json", DATA_PATH, name)
}

mod mock_engine {
    use std::time::Duration;
    use std::error::Error;

use crate::Song;

    pub struct Config{
        playlist_path: Option<String>,
        custom_name: Option<String>,
        custom_artist: Option<String>,
        custom_period_ms: Option<u64>,
    }
    
    pub struct Engine{
        config : Config,
        current_song: super::Song,
        current_song_id: String,
    }

    impl Engine {
        const DEFAULT_SONG_DURATION_MS : u64 = 3000;
        const DEFAULT_SONG_NAME : &str = "mock_name";
        const DEFAULT_SONG_ARTIST : &str = "mock_artist";
        const DEFAULT_SONG_ID : &str = "0";

        fn genegare_default_song() -> super::Song {
            super::Song{
                name: Some(String::from(Self::DEFAULT_SONG_NAME)),
                artist: Some(String::from(Self::DEFAULT_SONG_ARTIST)),
                duration_ms: Some(Self::DEFAULT_SONG_DURATION_MS),
                user_data: vec![ 
                    super::UserData{key: "key1".to_string(), value: "value1".to_string()},
                    super::UserData{key: "key2".to_string(), value: "value2".to_string()},
                ],
            }
        }

        pub fn new(config: Config) -> Engine {
            let engine = Engine {
                config: config,
                current_song: Self::genegare_default_song(),
                current_song_id: String::from(Self::DEFAULT_SONG_ID)
            };
            engine
        }

        pub async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>{
            if let Some(path) = &self.config.playlist_path {
                let playlist = super::load_playlist(path).expect("Could not load mock playlist.");

                // simulate looping playlist
                loop{
                    for (id, song) in playlist.iter(){
                        let period_ms = self.config.custom_period_ms.unwrap_or_else(|| {
                            song.duration_ms.unwrap_or(Self::DEFAULT_SONG_DURATION_MS)
                        });

                        tokio::time::sleep(Duration::from_secs(period_ms.clone())).await;

                        {
                            self.current_song = super::Song{
                                name: Some(self.config.custom_name.clone().unwrap_or_else(|| {
                                    song.name.clone().unwrap_or(String::from(Self::DEFAULT_SONG_NAME))
                                }
                                )),
                                artist: Some(self.config.custom_artist.clone().unwrap_or_else(|| {
                                    song.artist.clone().unwrap_or(String::from(Self::DEFAULT_SONG_ARTIST))
                                }
                                )),
                                duration_ms: Some(period_ms),
                                user_data: song.user_data.clone(),
                            };
                            self.current_song_id = id.clone();
                        }
                    }
                }
            }
            else{
                {
                    if let Some(name_override) = self.config.custom_name.clone() {self.current_song.name = Some(name_override)};
                    if let Some(artist_override) = self.config.custom_artist.clone() {self.current_song.artist = Some(artist_override)};
                    if let Some(duration_override) = self.config.custom_period_ms.clone() {self.current_song.duration_ms = Some(duration_override)};
                }
            }

            Ok(())
        }
    }
}

    // #todo replace with own scheduling system on a separate thread
    // make config based

/* #[cfg(test)]
mod tests {
    use super::*;
} */
