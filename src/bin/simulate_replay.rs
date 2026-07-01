use tokio;
use std::env;
use std::time::Duration;

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

    let config= snotify::mock::Config {
        playlist_path: Some(path),
        custom_artist: None,
        custom_name: None,
        custom_period_ms: Some(5000),
    };

    let mut engine = snotify::mock::Engine::new(config);
    engine.start();

    let mut current_id = String::from("");
    let mut consecutive_client_error_count = 0_usize;

    loop {
        match engine.current_playing().await {
            Some((song, id)) => {
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
            None => {
                consecutive_client_error_count+=1;

                if consecutive_client_error_count >= MAX_CLIENT_ERROR_COUNT {
                    println!("Max client error count reached. Stopping the app.");
                    break;
                }
            }
        }
    }
}
