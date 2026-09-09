use serde::Serialize;

const MAX_CLIENT_ERROR_COUNT: usize = 5;
const DEFAULT_SPOTIFY_REQUEST_PERIOD: u64 = 5000;
const ADDRESS_AND_PORT: &str = "127.0.0.1:6767";

#[derive(Serialize)]
struct Message {
    text: String,
    count: usize,
}

async fn hello() -> axum::Json<Message> {
    axum::Json(Message {
        text: "Hello from Rust".to_string(),
        count: 42,
    })
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!(
        "cargo run --bin record [playlist name] [key1] [value1] [key2] [value2] ... to update the playlist database"
    );

    let listener = tokio::net::TcpListener::bind(ADDRESS_AND_PORT)
        .await
        .unwrap();

    log::info!("listening on http://{}", listener.local_addr().unwrap());

    let app = axum::Router::new().route("/api/hello", axum::routing::get(hello));
    tokio::task::spawn(async move { axum::serve(listener, app).await.unwrap() });

    loop {}
    /*     let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 2,
        "Please provide a playlist name \
        Current arg length: {:?}",
        args
    );

    let path = snotify::make_playlist_path(&args[1]);

    let mut engine = snotify::Engine::new(path).unwrap_or_else(|error| {
        log::error!("Error: {}", error);
        process::exit(1);
    });

    let spotify_player = snotify::spotify::Player::new().await;
    let mut consecutive_client_error_count = 0_usize;

    loop {
        match spotify_player.get_currently_playing().await {
            Ok((song, id)) => {
                consecutive_client_error_count = 0;

                if engine.try_update(id) {
                    match engine.get_song_data() {
                        Some(song) => song.print_preview("Currently playing:"),
                        None => song.print_preview("Could not find database entry for song:"),
                    }
                }

                tokio::time::sleep(Duration::from_millis(DEFAULT_SPOTIFY_REQUEST_PERIOD)).await;
            }
            Err(error) => {
                log::error!("Error: {}", error);
                if let SnotifyError::ClientError(_) = error {
                    consecutive_client_error_count += 1;

                    if consecutive_client_error_count >= MAX_CLIENT_ERROR_COUNT {
                        log::error!("Max client error count reached. Stopping the app.");
                        process::exit(1);
                    }
                }

                if let Some(retry_after) = error.retry_after_s() {
                    tokio::time::sleep(Duration::from_millis(retry_after)).await;
                } else {
                    process::exit(1);
                }
            }
        }
    } */
}
