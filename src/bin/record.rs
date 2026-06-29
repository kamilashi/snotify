use snotify::UserData;
use tokio;
use rspotify::clients::OAuthClient;
use std::collections::HashMap;
use std::env;

/* fn try_log_song_data(name: &str, song: snotify::Song){
    std::fs::write("{str}.json", serde_json::to_string_pretty(&song)?)?;
} */

#[tokio::main]
async fn main()  {
    std::fs::create_dir_all(snotify::DATA_PATH);
    
    let spotify = snotify::authorize().await;

    let track = spotify.current_playing(None, None::<Vec<_>>).await.unwrap();

    //println!("{:#?}", track);

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

    song.user_data = Vec::new();
    for key_value in args[2..].chunks(2) {
        let data = UserData {
            key: key_value[0].clone(),
            value: key_value[1].clone(),
        };
        song.user_data.push(data);
    }

    // argument 0 is the binary path, so we skip it
    let path = snotify::make_playlist_path(&args[1]);
    let mut songs = match snotify::load_songs(&path) {
        Some(table) => {
            if(table.contains_key(&id)) {
                let song = table.get(&id).expect("should exist");
                song.print_preview("Overwriting data for song:");
            }
        table},
        None => HashMap::new()
    };

    songs.insert(id, song);

    std::fs::write(
        path,
        serde_json::to_string_pretty(&songs).expect("Could not serialize to .json")
    ).expect("Could not write to file");
}
