use std::{error::Error, sync::Arc, time::Duration};
use tokio::sync::watch;

pub struct Config{
    pub playlist_path: Option<String>,
    pub custom_name: Option<String>,
    pub custom_artist: Option<String>,
    pub custom_period_ms: Option<u64>,
}

struct CurrentSong{
    song: super::Song,
    id: String
}

pub struct Engine{
    engine : Arc<Impl>
}

impl Engine {
    pub fn new(config: Config) -> Engine{
        let engine = Engine {
                engine: Arc::new(Impl::new(config))
            };
        engine
    }

    pub fn start(&mut self){
        let engine = self.engine.clone();
        tokio::spawn(async move { engine.run().await });
        println!("Started mock player engine");
    } 

    pub async fn current_playing(&self) -> Option<(super::Song, String)> {
        Some(self.engine.get_song().await)
    }
}

struct Impl{
    config : Config,
    current_song_channel: watch::Sender<CurrentSong>,
}

impl Impl {
    const DEFAULT_SONG_DURATION_MS : u64 = 3000;
    const DEFAULT_SONG_NAME : &str = "mock_name";
    const DEFAULT_SONG_ARTIST : &str = "mock_artist";
    const DEFAULT_SONG_ID : &str = "0";

    fn genegare_default_song(config: &Config) -> super::Song {
        super::Song{
            name: Some(config.custom_name.clone().unwrap_or(String::from(Self::DEFAULT_SONG_NAME))),
            artist: Some(config.custom_artist.clone().unwrap_or(String::from(Self::DEFAULT_SONG_ARTIST))),
            duration_ms: Some(config.custom_period_ms.clone().unwrap_or(Self::DEFAULT_SONG_DURATION_MS)),
            user_data: vec![ 
                super::UserData{key: "key1".to_string(), value: "value1".to_string()},
                super::UserData{key: "key2".to_string(), value: "value2".to_string()},
            ],
        }
    }

    fn new(config: Config) -> Impl {
        let (tx, _rx) = watch::channel(CurrentSong{
            song: Self::genegare_default_song(&config),
            id: String::from(Self::DEFAULT_SONG_ID)
        });

        let engine = Impl {
            current_song_channel: tx,
            config: config,
        };
        engine
    }

    async fn get_song(&self) -> (super::Song, String) {
        let current = self.current_song_channel.borrow();
        (current.song.clone(), current.id.clone())
    }

    async fn run(&self) -> Result<(), Box<dyn Error + Send + Sync>>{
        if let Some(path) = &self.config.playlist_path {
            println!("Loading mock playlist {}", path);
            let playlist = super::load_playlist(path).expect("Could not load mock playlist.");

            // simulate looping playlist
            loop{
                for (id, song) in playlist.iter(){
                    let period_ms = self.config.custom_period_ms.unwrap_or_else(|| {
                        song.duration_ms.unwrap_or(Self::DEFAULT_SONG_DURATION_MS)
                    });

                    tokio::time::sleep(Duration::from_millis(period_ms.clone())).await;

                    {
                        let song_update = CurrentSong{
                            song: super::Song{
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
                            },
                            id: id.clone()
                        };

                        self.current_song_channel.send_replace(song_update); // don't care if there are any listeners
                        //song.print_preview("Replaying: ");
                    }
                }
            }
        }
        else{
            eprintln!("Cannot use run function without an actual playlist to run");
        }

        Ok(())
    }
}
