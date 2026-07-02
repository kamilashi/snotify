use std::{collections::HashMap, fmt};

// once there are proper runners this should disintegrate
pub use rspotify::{AuthCodeSpotify, Credentials, OAuth, model::{CurrentlyPlayingType::Unknown, PlayableItem, track}, prelude::*, scopes};
use serde::{Serialize, Deserialize};
pub use error_handling::{*};

pub mod error_handling;
pub mod mock;
pub mod spotify;

type Playlist = HashMap<String, Song>;

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
    pub user_data: Vec<UserData>, // #todo: use Option
}

// #todo get rid of
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

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  name: {}", self.name.as_deref().unwrap_or("unknown"))?;
        write!(f, "  artist: {}",self.artist.as_deref().unwrap_or("unknown"))?;

        for key_value in &self.user_data {
            write!(f, " {} : {}", key_value.key, key_value.value)?;
        }

        Ok(())
    }
}

pub fn load_playlist(path: String) -> Result<Playlist, SnotifyError> {
    if !std::path::Path::new(&path).exists() {
        return Err(SnotifyError::PathNotExistent(path))?;
    }

    let file = std::fs::read_to_string(path).map_err(|_| SnotifyError::FailedToReadFile)?;
    let map: Playlist = serde_json::from_str(&file).map_err(|_| SnotifyError::FailedDeserializeFromJson)?;
    Ok(map)
}

pub fn save_playlist(path: &str, songs: &Playlist){
    std::fs::write(
        path,
        serde_json::to_string_pretty(songs).expect("Could not serialize to .json")
    ).expect("Could not write to file");
}

pub fn make_playlist_path(name: &str) -> String{
    format!("{}{}.json", DATA_PATH, name)
}
pub struct Engine {
    playlist_database: Playlist,
    current_id: String
} 

impl Engine {
    pub fn new(path: String) -> Result<Self, SnotifyError> {
        let songs = load_playlist(path)?;

        Ok(Engine{
            playlist_database: songs,
            current_id: String::from("")
        })
    }
    
    #[must_use]
    pub fn try_update(&mut self, id: String) -> bool {
        if !self.current_id.eq(&id) {        
            self.current_id = id;
            return true;
        }
        false
    }

    pub fn get_song_data(&mut self) -> Option<Song> {
        return self.playlist_database.get(&self.current_id).cloned();
    }
}

