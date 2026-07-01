use std::env;
use std::time::Duration;

#[tokio::main]
async fn main()   {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 2,
        "Please provide a playlist name \
        Current arg length: {:?}", args
    );

    let path = snotify::make_playlist_path(&args[1]);
    let mock_playlist_path = snotify::make_playlist_path("test");
    assert!(std::path::Path::new(&path).exists(), "Database for playlist {} doesn't exist", &args[1]);

    let songs = snotify::load_playlist(&path).expect("Could not load song database");

    let config= snotify::mock::Config {
        playlist_path: Some(mock_playlist_path),
        custom_artist: None,
        custom_name: None,
        custom_period_ms: Some(5000),
    };

    let mut player = snotify::mock::Player::new(config);
    player.start();

    let mut current_id = String::from("");

    loop {
        let (song, id) = player.get_currently_playing().await;
        if !current_id.eq(&id) {        
            current_id = id;

            match songs.get(&current_id) {
                Some(song) => song.print_preview("Currently playing:"),
                None => song.print_preview("Could not find database entry for song:"),
            }
        }

        let sleep_for_ms = 3000_u64;
        tokio::time::sleep(Duration::from_millis(sleep_for_ms)).await;
    }
}
