use std::collections::HashMap;
use std::sync::Arc;
use crate::http_server::router::router::{RouteUrl};

pub type RouteFn = Arc<dyn Fn() -> String + Send + Sync>;

#[derive(Clone)]
pub struct PathRoute {
  pub routes: HashMap<RouteUrl, HashMap<RouteMethod, RouteFn>>
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum RouteMethod {
  GET,
  POST,
  PUT,
  DELETE,
}

impl PathRoute {
  pub fn route(&mut self, path: RouteUrl, method: RouteMethod, method_route: RouteFn) {
    let route = self.routes.entry(path).or_insert_with(HashMap::new);
    route.insert(method, method_route);
  }
}