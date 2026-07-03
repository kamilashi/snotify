use std::future::Future;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinSet;

pub struct Server {
    listener: TcpListener,
    tasks: JoinSet<()>,
    max_client_count: u32,
    current_connected_clients: u32,
}

type HttpRequest = (Vec<String>, Option<usize>);

impl Server {
    pub async fn new(address_port: &str, max_client_count: u32) -> Server {
        Server {
            listener: TcpListener::bind(address_port).await.unwrap(),
            tasks: JoinSet::new(),
            max_client_count,
            current_connected_clients: 0,
        }
    }

    pub async fn run<F, Fut>(&mut self, client_handler: F) -> Result<(), std::io::Error>
    where
        F: Fn(TcpStream) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        loop {
            let (stream, _addr) = self.listener.accept().await?;
            self.current_connected_clients += 1;
            self.tasks.spawn(client_handler(stream));

            if self.current_connected_clients == self.max_client_count {
                println!("Max client count reached!");
                break; // #todo :revisit
            }
        }

        while let Some(result) = self.tasks.join_next().await {
            if let Err(e) = result {
                eprintln!("Client task panicked: {e}");
            }
        }

        Ok(())
    }
}

pub struct Client {
    server_stream: TcpStream,
}

impl Client {
    pub async fn new(address_port: &str) -> Client {
        Client {
            server_stream: TcpStream::connect(address_port).await.unwrap(),
        }
    }

    pub fn get_server(&mut self) -> &mut TcpStream {
        &mut self.server_stream
    }
}

pub async fn read(stream: &mut TcpStream) -> Result<HttpRequest, std::io::Error> {
    let mut lines = BufReader::new(stream).lines();
    let mut header = Vec::new();
    let mut body_start_idx = None;
    let mut empty_lines_until_break = 1;

    while let Some(line) = lines.next_line().await? {
        if line.contains("Content-Length") {
            body_start_idx = Some(header.len() + 1);
            empty_lines_until_break = 2;
        }
        if line.is_empty() {
            empty_lines_until_break -= 1;
        } else {
            header.push(line);
        }
        if empty_lines_until_break == 0 {
            break;
        }
    }

    Ok((header, body_start_idx))
}

pub async fn write(stream: &mut TcpStream, msg: &str) -> Result<(), std::io::Error> {
    stream.write_all(format!("{msg}\r\n\r\n").as_bytes()).await
}
