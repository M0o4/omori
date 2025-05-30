pub enum StatusCode {
    OK,
    NotFound,
}

impl StatusCode {
    pub fn to_str(&self) -> &'static str {
        match self {
            StatusCode::OK => "200 OK",
            StatusCode::NotFound => "404 Not Found",
        }
    }
}
