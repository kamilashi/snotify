use snotify;
use std::time::Duration;
use std::{env, process};

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 2,
        "Please provide a playlist name \
        Current arg length: {:?}",
        args
    );

    let path = snotify::make_playlist_path(&args[1]);
    let mock_playlist_path = snotify::make_playlist_path("test");

    let mut engine = snotify::Engine::new(path).unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        process::exit(1);
    });

    let config = snotify::mock::Config {
        playlist_path: Some(mock_playlist_path),
        custom_artist: None,
        custom_name: None,
        custom_period_ms: Some(5000),
        debug_print: false,
    };

    let player = snotify::mock::Player::new(config);
    let _ = player.start_async();

    loop {
        let (song, id) = player.get_currently_playing();

        if engine.try_update(id) {
            match engine.get_song_data() {
                Some(song) => song.print_preview("Currently playing:"),
                None => song.print_preview("Could not find database entry for song:"),
            }
        }

        let sleep_for_ms = 3000_u64;
        tokio::time::sleep(Duration::from_millis(sleep_for_ms)).await;
    }
}
