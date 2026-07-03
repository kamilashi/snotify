use log::{debug, error, info, warn};
use snotify::mock::ipc;
use std::{net::TcpStream, sync::Arc};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut server = ipc::Server::new();

    let mock_playlist_path = snotify::make_playlist_path("test");

    let config = snotify::mock::Config {
        playlist_path: Some(mock_playlist_path),
        custom_artist: None,
        custom_name: None,
        custom_period_ms: Some(7000),
        debug_print: true,
    };

    let player = snotify::mock::Player::new(config);
    player.start_async();

    let mut served_client_count = 0_usize;
    let mut tasks = JoinSet::new();

    for stream in server.get_clients() {
        let stream = stream.unwrap();

        tasks.spawn(snotify::mock::ipc::serve_client(
            stream,
            player.clone_async(),
        ));

        served_client_count += 1;

        if served_client_count == snotify::mock::ipc::MAX_CLIENT_COUNT {
            println!("Max client number reached: {served_client_count}");
            break;
        }

        // #todo: implement disconnect
    }

    while let Some(result) = tasks.join_next().await {
        if let Err(e) = result {
            error!("Client task panicked: {e}");
        }
    }
}
