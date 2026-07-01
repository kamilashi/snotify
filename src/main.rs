
use tokio;
use rspotify::clients::OAuthClient;
use std::env;
use std::time::Duration;
use rspotify::ClientError;

const MAX_CLIENT_ERROR_COUNT: usize = 5_usize;

#[tokio::main]
async fn main()   {
    println!("cargo run --bin record [playlist name] [key1] [key2] ... to update the playlist database");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 2,
        "Please provide a playlist name \
        Current arg length: {:?}", args
    );

    let path = snotify::make_playlist_path(&args[1]);
    assert!(std::path::Path::new(&path).exists(), "Database for playlist {} doesn't exist", &args[1]);

    let songs = snotify::load_playlist(&path).expect("Could not load song database");
    let mut current_id = String::from("");

    let spotify = snotify::authorize().await;
    let mut consecutive_client_error_count = 0_usize;

    loop {
        match spotify.current_playing(None, None::<Vec<_>>).await {
            Ok(track) => {
                //println!("{:#?}", track);
                consecutive_client_error_count = 0;

                let context = track.expect("Could not get context");
                let item = context.item.expect("Could not get item from the context");
                let (song, id) = snotify::get_song(item).expect("Could not retrieve song data");

                if !current_id.eq(&id) {        
                    current_id = id;

                    if songs.contains_key(&current_id) {
                        let song = songs.get(&current_id).expect("should exist");
                        song.print_preview("Currently playing:");
                    }
                    else {
                        song.print_preview("Could not find database entry for song:");
                    }
                }

                let sleep_for_ms = 3000_u64;
                tokio::time::sleep(Duration::from_millis(sleep_for_ms)).await;
            },
            Err(error) => {
                consecutive_client_error_count+=1;

                if consecutive_client_error_count >= MAX_CLIENT_ERROR_COUNT {
                    println!("Max client error count reached. Stopping the app.");
                    break;
                }

                println!("{}", error);
                let mut retry_after_secs: Option<u64> = None;

                if let ClientError::Http(http_err) = &error {
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

                let wait_secs = retry_after_secs.unwrap_or(30); 
                println!("Retrying after {} seconds", wait_secs);
                tokio::time::sleep(Duration::from_secs(wait_secs)).await;
            }
        }
    }
}
