use crate::http_server::router::path_route::{PathRoute, RouteFn, RouteMethod};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RouteId(u32);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RouteUrl(String);

pub struct Router {
  pub inner: Arc<RouterInner>,
}

struct RouterInner {
  path_route: PathRoute,
}

impl Router {
  pub fn new() -> Self {
    Self {
      inner: Arc::new(RouterInner {
        path_route: PathRoute {
          routes: HashMap::new(),
        },
      }),
    }
  }

  pub fn route<F>(
    self,
    path: &str,
    method: RouteMethod,
    method_route: F,
  ) -> Self
  where
    F: Fn() -> String + Send + Sync + 'static,
  {
    let mut inner = self.into_inner();
    inner.path_route.route(
      RouteUrl(path.to_string()),
      method,
      Arc::new(method_route),
    );

    Self {
      inner: Arc::new(inner),
    }
  }
  
  pub fn get_route(&self, path: &str, method: RouteMethod) -> RouteFn {
    let route_url = RouteUrl(path.to_string());

    if let Some(methods) = self.inner.path_route.routes.get(&route_url) {
      if let Some(handler) = methods.get(&method) {
        return handler.clone();
      }
    }
    
    Arc::new(|| "".to_string())
  }

  fn into_inner(self) -> RouterInner {
    Arc::try_unwrap(self.inner).unwrap_or_else(|arc| RouterInner {
      path_route: arc.path_route.clone(),
    })
  }
}
