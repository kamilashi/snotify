
use tokio;
use rspotify::clients::OAuthClient;
use std::env;

#[tokio::main]
async fn main()   {
    println!("cargo run --bin record [playlist name] [key1] [key2] ... to update the playlist database");

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 2,
        "Please provide a playlist name \
        Current arg length: {:?}", args
    );

    let path = snotify::make_playlist_path(&args[1]);

    assert!(std::path::Path::new(&path).exists(), "Database for playlist {} doesn't exist", &args[1]);

    let songs = snotify::load_songs(&path).expect("Could not load song database");

    let spotify = snotify::authorize().await;

    let track = spotify.current_playing(None, None::<Vec<_>>).await.unwrap();

    //println!("{:#?}", track);

    let context = track.expect("Could not get context");

    let item = context.item.expect("Could not get item from the context");

    let (song, id) = snotify::get_song(item).expect("Could not retrieve song data");

    if songs.contains_key(&id) {
        println!("Currently playing:");
        let song = songs.get(&id).expect("should exist");
        song.print_preview("Currently playing:");
    }
    else {
        song.print_preview("Could not find database entry for song:");
    }
}
