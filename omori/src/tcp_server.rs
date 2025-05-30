use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::{io, thread};

pub struct TcpServer {
    host: String,
    port: u16,
}

impl TcpServer {
    pub fn new(host: &str, port: u16) -> TcpServer {
        TcpServer {
            host: host.to_string(),
            port,
        }
    }
}

impl Server for TcpServer {
    fn start<T: StreamHandler + Send + Sync + 'static>(&self, stream_handel: T) {
        let listener =
          TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap_or_else(|err| {
              panic!("Failed to bind TcpListener, error: {}", err);
          });

        let handler = Arc::new(stream_handel);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let handler = handler.clone();
                    thread::spawn(move || {
                        handler
                          .handle_client(stream)
                          .unwrap_or_else(|error| eprintln!("{}", error));
                    });
                }
                Err(_) => {}
            }
        }
    }
}

pub trait Server {
    fn start<T: StreamHandler + Send + Sync + 'static>(&self, stream_handel: T);
}

pub trait StreamHandler {
    fn handle_client(&self, stream: TcpStream) -> Result<(), io::Error>;
}

pub struct TcpHandler {}

impl TcpHandler {
    pub fn new() -> TcpHandler {
        TcpHandler {}
    }
}

impl StreamHandler for TcpHandler {
    fn handle_client(&self, mut stream: TcpStream) -> Result<(), io::Error> {
        let mut buffer = [0; 4096];

        loop {
            let buffer_size = stream.read(&mut buffer)?;

            if buffer_size == 0 {
                return Ok(());
            }

            let buffer_string = match String::from_utf8(buffer[..buffer_size].to_vec()) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Invalid UTF-8 sequence: {}", e);
                    continue;
                }
            };

            println!("{}", buffer_string);

            stream.write_all(&buffer[0..buffer_size])?;
            stream.flush()?
        }
    }
}
