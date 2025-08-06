use omori::http_server::router::path_route::RouteMethod;
use omori::http_server::router::router::Router;
use omori::http_server::server::HTTPServer;

fn main() {
  let app = Router::new().route("/", RouteMethod::GET, root);
  let http_server = HTTPServer::new("0.0.0.0", 8080);
  http_server.serve(app);
}

fn root() -> String {
  "Hello, World!".to_string()
}
