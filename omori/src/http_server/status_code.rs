pub enum StatusCode {
  OK,
  Created,
  NoContent,
  NotFound,
  BadRequest,
  Forbidden,
  NotImplemented,
  InternalServerError,
}

impl StatusCode {
  pub fn to_str(&self) -> &'static str {
    match self {
      StatusCode::OK => "200 OK",
      StatusCode::Created => "201 Created",
      StatusCode::NoContent => "204 No Content",
      StatusCode::BadRequest => "400 Bad Request",
      StatusCode::Forbidden => "403 Forbidden",
      StatusCode::NotFound => "404 Not Found",
      StatusCode::InternalServerError => "500 Internal Server Error",
      StatusCode::NotImplemented => "501 Not Implemented",
    }
  }
}
