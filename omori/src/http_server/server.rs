use crate::http_server::handler::HttpHandler;
use crate::tcp_server::{Server, StreamHandler, TcpServer};

pub struct HTTPServer {
  tcp_server: TcpServer,
}

impl HTTPServer {
  pub fn new(host: &str, port: u16) -> HTTPServer {
    HTTPServer {
      tcp_server: TcpServer::new(host, port),
    }
  }

  pub fn serve(&self) {
    self.start(HttpHandler::new())
  }
}

impl Server for HTTPServer {
  fn start<T: StreamHandler + Send + Sync + 'static>(&self, stream_handel: T) {
    self.tcp_server.start(stream_handel)
  }
}
