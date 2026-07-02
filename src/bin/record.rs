use snotify::UserData;
use tokio;
use std::env;

#[tokio::main]
async fn main()  {
    env_logger::init();
    std::fs::create_dir_all(snotify::DATA_PATH).unwrap();
    
    let spotify_player = snotify::spotify::Player::new().await;
    let (mut song, id) = spotify_player.get_currently_playing().await.expect("Could not fetch the current song");

    song.print_preview("Currently playing - ");

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() >= 4 && args.len() % 2 == 0,
        "Please provide a playlist name and an even \
        number of subsequent arguments: key1 value1 key2 value2 ... . \n \
        Current arg length: {:?}", args
    );

    for key_value in args[2..].chunks(2) {
        let data = UserData {
            key: key_value[0].clone(),
            value: key_value[1].clone(),
        };
        song.user_data.push(data);
    }

    // argument 0 is the binary path, so we skip it
    let path = snotify::make_playlist_path(&args[1]);
    let mut songs = snotify::load_playlist(&path).unwrap_or_default();
    if let Some(existing) = songs.get(&id) {
        existing.print_preview("Overwriting data for song:");
    }

    songs.insert(id, song);

    snotify::save_playlist(&path, &songs);
}
