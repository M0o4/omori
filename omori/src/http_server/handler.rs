use crate::http_server::status_code::StatusCode;
use crate::tcp_server::StreamHandler;
use std::io::{Error, Read, Write};
use std::net::TcpStream;

pub struct HttpHandler {}

impl HttpHandler {
    pub fn new() -> HttpHandler {
        HttpHandler {}
    }
}

impl StreamHandler for HttpHandler {
    fn handle_client(&self, mut stream: TcpStream) -> Result<(), Error> {
        let mut buffer = [0; 4096];

        loop {
            let buffer_size = stream.read(&mut buffer)?;

            if buffer_size == 0 {
                return Ok(());
            }

            let response_body = b"\
{
    \"value\": \"Hello, World!\"
}";

            let response_line = format!("HTTP/1.1 {}\r\n", StatusCode::OK.to_str());
            let headers = format!(
                "Content-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nConnection: keep-alive\r\nKeep-Alive: timeout=5\r\nX-Powered-By: Rust\r\n",
                response_body.len()
            );
            let blank_line = b"\r\n";

            stream.write_all(response_line.as_bytes())?;
            stream.write_all(headers.as_bytes())?;
            stream.write_all(blank_line)?;
            stream.write_all(response_body)?;
            stream.flush()?
        }
    }
}
