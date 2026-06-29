use tokio;
use rspotify::{AuthCodeSpotify, Credentials, OAuth, model::{CurrentlyPlayingType::Unknown, PlayableItem, track}, prelude::*, scopes};

struct Song {
    name: String,
    artist: String,
}

impl Song {
    fn print_preview(&self){
        println!("Song");
        println!("  name: {}", self.name);
        println!("  artist: {}",self.artist);
    }
}

async fn authorize() -> AuthCodeSpotify {
    env_logger::init();

    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    spotify
}

fn get_song(item: PlayableItem) -> Option<(Song, String)>{
     match item {
        PlayableItem::Unknown(object) => {
            let song = Song {
                name: String::from("wip"),
                artist: String::from("wip"),
            };
            let id = String::from("wip");
            Some((song, id))
        }
        unhandled => {
            println!("Unimplemented playback item type {:#?}", unhandled);
            None
        }
    }
}

#[tokio::main]
async fn main()  {
    println!("Connect to spotify");
    
    let spotify = authorize().await;

    let track = spotify.current_playing(None, None::<Vec<_>>).await.unwrap();

    println!("{:#?}", track);

    let context = track.expect("Could not get context");

    let item = context.item.expect("Could not get item from the context");

    let (song, id) = get_song(item).expect("Could not retrieve song data");

    song.print_preview();
}
