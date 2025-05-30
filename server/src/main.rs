use omori::http_server::server::HTTPServer;

fn main() {
    let http_server = HTTPServer::new("0.0.0.0", 8080);
    http_server.serve();
}
