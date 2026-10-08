use serde::Serialize;
use snotify::error_handling::*;
use std::time::Duration;
use std::{env, process};
use tower_http::services::{ServeDir, ServeFile};

const MAX_CLIENT_ERROR_COUNT: usize = 5;
const DEFAULT_SPOTIFY_REQUEST_PERIOD: u64 = 5000;
const ADDRESS_AND_PORT: &str = "127.0.0.1:6767";

#[derive(Serialize)]
struct Message {
    text: String,
    count: usize,
}

async fn get_song(
    axum::extract::State(rx): axum::extract::State<
        tokio::sync::watch::Receiver<Option<snotify::Song>>,
    >,
) -> axum::Json<Option<snotify::Song>> {
    axum::Json(rx.borrow().clone())
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

    let addresss = listener.local_addr().unwrap();
    let (tx, rx) = tokio::sync::watch::channel(None::<snotify::Song>);

    let app = axum::Router::new()
        .route("/snotify", axum::routing::get(get_song))
        .with_state(rx)
        .fallback_service(ServeDir::new("static").fallback(ServeFile::new("static/index.html")));

    tokio::task::spawn(async move { axum::serve(listener, app).await.unwrap() });

    log::info!("listening on http://{}/snotify", addresss);

    let args: Vec<String> = env::args().collect();
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
                        Some(song) => {
                            song.print_preview("Currently playing:");
                            tx.send_replace(Some(song));
                        }
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
    }
}
