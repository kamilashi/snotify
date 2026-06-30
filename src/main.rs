
use tokio;
use rspotify::clients::OAuthClient;
use std::{env, thread};
use std::time::Duration;

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
    let mut current_id = String::from("");

    loop {
        let track = spotify.current_playing(None, None::<Vec<_>>).await.unwrap();

        //println!("{:#?}", track);

        let context = track.expect("Could not get context");
        let item = context.item.expect("Could not get item from the context");
        let (song, id) = snotify::get_song(item).expect("Could not retrieve song data");

        if current_id.eq(&id) {        
            let mut sleep_for_ms = 500_u64;

            // if the song hasn't transitoined yet, wait until it SHOULD transition - works with raw durations,
            // does not account for manual transitions
            /* if let (Some(progress), Some(duration)) = (context.progress, song.duration_ms) {
                sleep_for_ms = duration - (progress.num_milliseconds() as u64);
            } */

            thread::sleep(Duration::from_millis(sleep_for_ms));
            continue;
        }

        current_id = id;

        if songs.contains_key(&current_id) {
            let song = songs.get(&current_id).expect("should exist");
            song.print_preview("Currently playing:");
        }
        else {
            song.print_preview("Could not find database entry for song:");
        }
    }
}
