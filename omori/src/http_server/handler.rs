use crate::http_server::status_code::StatusCode;
use crate::tcp_server::StreamHandler;
use std::io::{Error, ErrorKind, Read, Write};
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

      let buffer_string =
        match String::from_utf8(buffer[..buffer_size].to_vec()) {
          Ok(string) => string,
          Err(_) => {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid UTF-8"));
          }
        };

      let http_request = buffer_string.split("\r\n").collect::<Vec<&str>>();
      let method_path = http_request[0].split(" ").collect::<Vec<&str>>();

      println!("[HTTP] {} {}", method_path[0], method_path[1]);

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
