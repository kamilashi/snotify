use super::*;
use axum::Router;
use tokio::{
    net::TcpListener,
    sync::watch::{channel, Receiver, Sender},
};
use tower_http::services::{ServeDir, ServeFile};

const ADDRESS_AND_PORT: &str = "127.0.0.1:6767";
const BASE_API_ENDPOINT: &str = "snotify";

// #todo: do not use snotify type here, pass serialized Json opbject
async fn get_song(
    axum::extract::State(rx): axum::extract::State<tokio::sync::watch::Receiver<Option<Song>>>,
) -> axum::Json<Option<Song>> {
    axum::Json(rx.borrow().clone())
}

fn create_router(rx: Receiver<Option<Song>>) -> Router {
    axum::Router::new()
        .route(
            &format!("/{}", BASE_API_ENDPOINT),
            axum::routing::get(get_song),
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

    log::info!("listening on http://{}/{}", addresss, BASE_API_ENDPOINT);

    tx
}
