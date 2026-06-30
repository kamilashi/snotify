use std::collections::HashMap;

pub use rspotify::{AuthCodeSpotify, Credentials, OAuth, model::{CurrentlyPlayingType::Unknown, PlayableItem, track}, prelude::*, scopes};
use serde::{Serialize, Deserialize};

pub const DATA_PATH: &str = "data/";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
    env_logger::init();

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

pub fn load_songs(path: &str) -> Option<HashMap<String, Song>> {
    let file = std::fs::read_to_string(path).ok()?;
    let map: HashMap<String, Song> = serde_json::from_str(&file).ok()?;
    Some(map)
}

pub fn make_playlist_path(name: &str) -> String{
    format!("{}{}.json", DATA_PATH, name)
}
