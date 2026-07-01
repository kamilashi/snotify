use std::collections::HashMap;

// once there are proper runners this should disintegrate
pub use rspotify::{AuthCodeSpotify, Credentials, OAuth, model::{CurrentlyPlayingType::Unknown, PlayableItem, track}, prelude::*, scopes};
use serde::{Serialize, Deserialize};

pub const DATA_PATH: &str = "data/";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserData {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Song {
    pub name: Option<String>,
    pub artist: Option<String>,
    pub duration_ms: Option<u64>,
    pub user_data: Vec<UserData>,
}

impl Song {
    pub fn print_preview(&self, prefix_msg: &str){
        println!("{}", prefix_msg);
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

pub mod mock;

/* #[cfg(test)]
mod tests {
    use super::*;
} */
