use log::{debug, error, info, warn};
use snotify;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut server = snotify::ipc::Server::new(
        snotify::mock::ipc::IP_AND_PORT,
        snotify::mock::ipc::MAX_CLIENT_COUNT,
    )
    .await;

    let mock_playlist_path = snotify::make_playlist_path("test");

    let config = snotify::mock::Config {
        playlist_path: Some(mock_playlist_path),
        custom_artist: None,
        custom_name: None,
        custom_period_ms: Some(7000),
        debug_print: true,
    };

    let player = snotify::mock::Player::new(config);
    player.start_async().expect("Could not start player");

    server
        .run(|stream| {
            let shared_player = player.clone_async();
            async move { snotify::mock::ipc::serve_client(stream, shared_player).await }
        })
        .await
        .expect("Server crashed");
    // #todo: implement disconnect
}
