use snotify::{deserialize_json, mock::ipc};
use std::{clone, env, process};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 2,
        "Please provide a playlist name \
        Current arg length: {:?}",
        args
    );

    let path = snotify::make_playlist_path(&args[1]);

    let mut engine = snotify::Engine::new(path).unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        process::exit(1);
    });

    let mut client = ipc::Client::new();
    let server = client.get_server();
    let request = format!("GET / HTTP/1.1 \n Host: {} \r\n\r\n", ipc::IP_AND_PORT);

    loop {
        ipc::write(server, &request).unwrap();

        let (all_lines, content_idx) = ipc::read(server);

        if let Some(content_start) = content_idx {
            let current_song =
                deserialize_json::<ipc::CurrentSong>(&all_lines[content_start..].join("\n"))
                    .unwrap();

            if engine.try_update(current_song.id) {
                match engine.get_song_data() {
                    Some(song) => song.print_preview("Currently playing:"),
                    None => current_song
                        .song
                        .print_preview("Could not find database entry for song:"),
                }
            }

            std::thread::sleep(std::time::Duration::from_secs(1)); // poll every second
        }
    }
}
