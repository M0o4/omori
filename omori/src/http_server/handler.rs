use std::fs;
use crate::http_server::status_code::StatusCode;
use crate::tcp_server::StreamHandler;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use crate::http_server::router::path_route::RouteMethod;
use crate::http_server::router::router::Router;

pub struct HttpHandler {
  router: Router,
}

impl HttpHandler {
  pub fn new(router: Router) -> HttpHandler {
    HttpHandler {
      router,
    }
  }
}

impl StreamHandler for HttpHandler {
  fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Error> {
    
    loop {
      let buffer_reader = BufReader::new(&stream);
      let request_line = match buffer_reader.lines().next() {
        Some(request_line) => request_line?,
        None => Err(Error::from(ErrorKind::UnexpectedEof))?,
      };
      
      let method_path = request_line.split(" ").collect::<Vec<&str>>();
      
      let route = self.router.get_route( method_path[0], RouteMethod::GET);
      let res = route();

      println!("[HTTP] {} {}", method_path[0], method_path[1]);
      
      if request_line == "GET / HTTP/1.1" {
        let response_body = fs::read_to_string("hello.html")?;

        let response_line = format!("HTTP/1.1 {}\r\n", StatusCode::OK.to_str());
        let headers = format!(
          "Content-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: keep-alive\r\nKeep-Alive: timeout=5\r\nX-Powered-By: Rust\r\n",
          response_body.len()
        );
        let blank_line = b"\r\n";

        stream.write_all(response_line.as_bytes())?;
        stream.write_all(headers.as_bytes())?;
        stream.write_all(blank_line)?;
        stream.write_all(response_body.as_bytes())?;
      } else {
        let response_body = b"\
{
  \"error\": \"invalid_request\",
  \"success\": false
}
 ";
        let response_line = format!("HTTP/1.1 {}\r\n", StatusCode::NotImplemented.to_str());
        let headers = format!(
          "Content-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nConnection: keep-alive\r\nKeep-Alive: timeout=5\r\nX-Powered-By: Rust\r\n",
          response_body.len()
        );
        let blank_line = b"\r\n";

        stream.write_all(response_line.as_bytes())?;
        stream.write_all(headers.as_bytes())?;
        stream.write_all(blank_line)?;
        stream.write_all(response_body)?;
      }


      stream.flush()?
    }
  }
}
