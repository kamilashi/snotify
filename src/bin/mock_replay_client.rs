use std::{env, process};

fn main()   {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 2,
        "Please provide a playlist name \
        Current arg length: {:?}", args
    );

    let path = snotify::make_playlist_path(&args[1]);

    let mut engine = snotify::Engine::new(path).unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        process::exit(1);
    });

    loop {

        if engine.try_update(id) {
            match engine.get_song_data() {
                Some(song) => song.print_preview("Currently playing:"),
                None => song.print_preview("Could not find database entry for song:"),
            }
        } 
    }
}
