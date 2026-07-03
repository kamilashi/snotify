use snotify;
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

    let mut engine = snotify::Engine::new(path).unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        process::exit(1);
    });

    let mut client = snotify::ipc::Client::new(snotify::mock::ipc::IP_AND_PORT).await;
    let server = client.get_server();
    let request = format!(
        "GET / HTTP/1.1 \n Host: {} \r\n\r\n",
        snotify::mock::ipc::IP_AND_PORT
    );

    loop {
        snotify::ipc::write(server, &request).await.unwrap();

        let (all_lines, content_idx) = snotify::ipc::read(server).await.unwrap();

        if let Some(content_start) = content_idx {
            let current_song = snotify::deserialize_json::<snotify::mock::ipc::CurrentSong>(
                &all_lines[content_start..].join("\n"),
            )
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
