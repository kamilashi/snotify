use std::process;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use snotify::mock::ipc;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}

#[tokio::main]
async fn main()   {
    env_logger::init();

    let listener = TcpListener::bind(ipc::IP_AND_PORT).unwrap();

    let mock_playlist_path = snotify::make_playlist_path("test");

    let config= snotify::mock::Config {
        playlist_path: Some(mock_playlist_path),
        custom_artist: None,
        custom_name: None,
        custom_period_ms: Some(5000),
        debug_print: true,
    };

    let player = snotify::mock::Player::new(config);
    player.start_async();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect(); 
        println!("Request: {http_request:#?}");

        let (song, _) = player.get_currently_playing();
        stream.write_all(snotify::serialize_json(&song).map_err(|err| {
            eprintln!("Error: {err}")
        }).unwrap().as_bytes()).unwrap();
    }
}
