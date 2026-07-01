use snotify::UserData;
use tokio;
use rspotify::clients::OAuthClient;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main()  {
    env_logger::init();
    std::fs::create_dir_all(snotify::DATA_PATH).unwrap();
    
    let spotify = snotify::authorize().await;
    let track = spotify.current_playing(None, None::<Vec<_>>).await.unwrap();

    dbg!(&track);

    let context = track.expect("Could not get context");
    let item = context.item.expect("Could not get item from the context");
    let (mut song, id) = snotify::get_song(item).expect("Could not retrieve song data");

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
