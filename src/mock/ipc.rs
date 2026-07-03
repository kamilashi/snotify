pub const IP_AND_PORT: &str = "127.0.0.1:7878";

// #todo: make runtime agnostic
use std::{
    io::{prelude::*, BufReader},
    net::{Incoming, TcpListener, TcpStream},
};

use super::*;

#[derive(Serialize, Deserialize)]
pub struct CurrentSong {
    pub song: Song,
    pub id: String,
}

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new() -> Server {
        Server {
            listener: TcpListener::bind(IP_AND_PORT).unwrap(),
        }
    }

    pub fn get_clients(&mut self) -> Incoming<'_> {
        self.listener.incoming()
    }
}

pub struct Client {
    server_stream: TcpStream,
}

impl Client {
    pub fn new() -> Client {
        Client {
            server_stream: TcpStream::connect(IP_AND_PORT).unwrap(),
        }
    }

    pub fn get_server(&mut self) -> &mut TcpStream {
        &mut self.server_stream
    }
}

pub fn serialize_html_responcel(code: &str, status: &str, msg: String) -> String {
    format!(
        "HTTP/1.1 {code} {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        msg.len(),
        msg
    )
}

/*pub fn deserialize_html_responcel(msg: &Vec<String>) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        msg.len(),
        msg
    )
} */

pub fn read(stream: &TcpStream) -> (Vec<String>, Option<usize>) {
    let mut parts = 1;
    let mut body_start_idx = None;
    let mut line_idx = 0_usize;

    let buf_reader = BufReader::new(stream);
    let header: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| {
            if line.contains("Content-Length") {
                parts = 2;
                body_start_idx = Some(line_idx + 2);
            }

            if line.is_empty() {
                parts -= 1;
            }

            line_idx += 1;
            parts > 0
        })
        .collect();

    (header, body_start_idx)
}

pub fn write(stream: &mut TcpStream, msg: &str) -> Result<(), std::io::Error> {
    stream.write_all(format!("{msg}\r\n\r\n").as_bytes())
}
