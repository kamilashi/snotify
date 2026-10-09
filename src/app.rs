use std::time::Duration;

use async_stream::try_stream;

use super::*;
use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::get,
    Router,
};
use futures::Stream;
use tokio::{
    net::TcpListener,
    sync::watch::{channel, Receiver, Sender},
};
use tower_http::services::{ServeDir, ServeFile};

const ADDRESS_AND_PORT: &str = "127.0.0.1:6767";
const BASE_API_ENDPOINT: &str = "snotify";

async fn get_song_stream(
    axum::extract::State(mut rx): axum::extract::State<Receiver<Option<Song>>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    Sse::new(try_stream! {
        loop {
            let song = rx.borrow_and_update().clone();
            if let Some(song) = song {
                yield Event::default().json_data(song)?;
            }
            if rx.changed().await.is_err() {
                break;
            }
         }
    })
    .keep_alive(KeepAlive::default())
}

fn create_router(rx: Receiver<Option<Song>>) -> Router {
    Router::new()
        .route(
            &format!("/{}/current", BASE_API_ENDPOINT),
            get(get_song_stream),
        )
        .with_state(rx)
        .fallback_service(ServeDir::new("static").fallback(ServeFile::new("static/index.html")))
}

pub async fn create_backend() -> Sender<Option<Song>> {
    let listener = TcpListener::bind(ADDRESS_AND_PORT).await.unwrap();

    let addresss = listener.local_addr().unwrap();
    let (tx, rx) = channel(None::<Song>);

    let router = create_router(rx);

    tokio::task::spawn(async move { axum::serve(listener, router).await.unwrap() });

    log::info!("serving on http://{}", addresss);

    tx
}
