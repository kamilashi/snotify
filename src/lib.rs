use std::{collections::HashMap, fmt};

// once there are proper runners this should disintegrate
pub use rspotify::{AuthCodeSpotify, Credentials, OAuth, model::{CurrentlyPlayingType::Unknown, PlayableItem, track}, prelude::*, scopes};
use serde::{Serialize, Deserialize};

pub const DATA_PATH: &str = "data/";

#[derive(Debug)]
pub struct ErrorWithRetryAfter{
   pub error: Box<dyn std::error::Error>,
   pub retry_after_s: u64
}

#[derive(Debug)]
pub struct RetryAfter{
   pub s: u64
}

#[derive(Debug)]
pub enum SnotifyError {
    ClientError(ErrorWithRetryAfter),
    UnsupportedItemType((PlayableItem, RetryAfter)),
    NoPlayableItem(RetryAfter),
    MissingStringId((Song, RetryAfter)),
    NoCurrentlyPlayingContext,
    Unknown,
}

impl fmt::Display for SnotifyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ClientError(error_with_retry) => {
                write!(f, "Client error: \n {}", error_with_retry.error)?;
                write!(f, "Retry after {} seconds", error_with_retry.retry_after_s)
            },
            Self::UnsupportedItemType((item, retry_after)) => {
                write!(f, "The handling of playable item {:#?} is not implemented.", item)?;
                write!(f, "Retry after {} seconds", retry_after.s)
            },
            Self::NoPlayableItem(retry_after_s) => {
                write!(f, "Failed to fetch playable item.")?;
                write!(f, "Retry after {} seconds",retry_after_s.s)
            },
            Self::MissingStringId((song, retry_after_s)) => {
                write!(f, "Missing id on song {} ", song)?;
                write!(f, "Retry after {} seconds", retry_after_s.s)
            },
            Self::NoCurrentlyPlayingContext => {
                write!(f, "Failed to fetch currently playing context.")
            },
            Self::Unknown => {
                write!(f, "Encountered an unknown error.")
            }
        }
    }
}

impl SnotifyError {
    pub fn retry_after_s(&self) -> Option<u64> {
        match self {
            Self::ClientError(error_with_retry) => {
                Some(error_with_retry.retry_after_s)
            },
            Self::UnsupportedItemType((_, retry_after_s)) => {
                Some(retry_after_s.s)
            },
            Self::NoPlayableItem(retry_after_s) => {
                Some(retry_after_s.s)
            },
            Self::MissingStringId((_, retry_after_s)) => {
                Some(retry_after_s.s)
            },
            Self::NoCurrentlyPlayingContext => {
                None
            },
            Self::Unknown => {
                None
            }
        }
    } 
}

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
pub mod spotify;

