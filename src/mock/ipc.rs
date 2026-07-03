pub const IP_AND_PORT: &str = "127.0.0.1:7878";
pub const MAX_CLIENT_COUNT: u32 = 10;

use tokio::net::TcpStream;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct CurrentSong {
    pub song: Song,
    pub id: String,
}

pub fn serialize_html_responcel(code: &str, status: &str, msg: String) -> String {
    format!(
        "HTTP/1.1 {code} {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        msg.len(),
        msg
    )
}

pub async fn serve_client(mut stream: TcpStream, player: Arc<ArcPlayer>) {
    loop {
        let request = crate::ipc::read(&mut stream).await.unwrap();
        println!("Request: {request:#?}");

        let (song, id) = player.get_currently_playing();
        let current_song = CurrentSong { song, id };

        let message = serialize_html_responcel(
            "200",
            "OK",
            serialize_json(&current_song)
                .map_err(|err| eprintln!("Error: {err}"))
                .expect("Could not serialize"),
        );

        crate::ipc::write(&mut stream, &message).await.unwrap();
    }
}
