use super::{*};
use rspotify::clients::OAuthClient;

const DEFAULT_RETRY_AFTER_S : u64 = 30;
pub struct Player{
    spotify: AuthCodeSpotify
}

async fn authorize() -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
    let spotify: AuthCodeSpotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    spotify
}

fn get_song_id<T: fmt::Display>(song: Song, id: Option<T>) -> Result<(Song, String), SnotifyError>{
    match id {
        Some(id) => Ok((song, id.to_string())),
        None => {
            let retry = RetryAfter {
                s: song.duration_ms.unwrap_or(DEFAULT_RETRY_AFTER_S),
            };
            return Err(SnotifyError::MissingStringId((song, retry)));
        }
    }
}

fn get_song(item: PlayableItem) -> Result<(Song, String), SnotifyError>{
     match item {
        PlayableItem::Unknown(object) => {
            let song = Song {
                name: object["name"].as_str().map(String::from),
                artist: object["artists"][0]["name"].as_str().map(String::from),
                duration_ms: object["duration_ms"].as_u64(),
                user_data: Vec::new()
            };
            get_song_id(song, object["id"].as_str())
        }
        PlayableItem::Track(track) => {
            let song = Song {
                name: Some(track.name),
                artist: Some(track.artists[0].name.clone()),
                duration_ms: Some(track.duration.num_milliseconds() as u64),
                user_data: Vec::new()
            };
            get_song_id(song, track.id)
        },
        unhandled => {
            Err(SnotifyError::UnsupportedItemType( (unhandled, RetryAfter { s: DEFAULT_RETRY_AFTER_S })))
        }
    }
}

impl Player {
    pub async fn new() -> Player{
        Player {
            spotify: authorize().await
        }
    }

    pub async fn get_currently_playing(&self) -> Result<(Song, String), SnotifyError>{
        println!("DEBUG REQUESTING SPOTIFY");
         match self.spotify.current_playing(None, None::<Vec<_>>).await {
            Ok(track) => {
                let context = track.ok_or(SnotifyError::NoCurrentlyPlayingContext)?;
                let item = context.item.ok_or(SnotifyError::NoPlayableItem(RetryAfter { s: DEFAULT_RETRY_AFTER_S }))?;
                get_song(item)
            },
            Err(error) => {
                let mut retry_after_secs: Option<u64> = None;

                if let rspotify::ClientError::Http(http_err) = &error {
                    if let rspotify_http::HttpError::StatusCode(response) = &**http_err {
                        if response.status().as_u16() == 429 {
                            retry_after_secs = response
                                .headers()
                                .get(reqwest::header::RETRY_AFTER)
                                .and_then(|v| v.to_str().ok())
                                .and_then(|v| v.parse::<u64>().ok());
                        }
                    }
                }

                Err(SnotifyError::ClientError( ErrorWithRetryAfter {
                    error: Box::new(error),
                    retry_after_s: retry_after_secs.unwrap_or(DEFAULT_RETRY_AFTER_S),
                }))
            }
        }
    }
}
