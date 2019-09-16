use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Status {
    OK,
    MovedPermanently,
    Found,
    BadRequest,
    InternalServerError,
}

impl Eq for Status {

}

impl Hash for Status {
    fn hash<H: Hasher>(&self, state: &mut H) {
    }
}

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: String,
}

lazy_static! {
    pub(crate) static ref STATUS_MAP: HashMap<Status, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        m.insert(Status::OK,("200","OK"));
        m.insert(Status::MovedPermanently,("301","Moved Permanently"));
        m.insert(Status::Found,("302","Found"));
        m.insert(Status::BadRequest,("400","Bad Request"));
        m.insert(Status::InternalServerError,("500","Internal Server Error"));
        m
    };
}

impl Response {
    pub(crate) fn new() -> Self {
        Response {
            status: Status::OK,
            headers: HashMap::new(),
            body: "".to_string(),
        }
    }
    pub(crate) fn marshal(&self) -> String {
        let (status_code, status_name) = STATUS_MAP.get(&self.status).unwrap();
        let status_line = format!("HTTP/1.1 {} {}\r\n", status_code, status_name);
        let headers = self.headers.iter().map(|(key, value)| {
            format!("{}: {}", key, value)
        }).collect::<Vec<String>>().join("\r\n");
        return status_line + headers.as_str() + "\r\n\r\n" + self.body.as_str();
    }
}