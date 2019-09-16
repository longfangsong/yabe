use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
}

#[derive(Debug)]
pub struct URI {
    pub path: String,
    pub query: HashMap<String, String>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum HTTPVersion {
    Http10,
    Http11,
    Http20,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: URI,
    pub headers: HashMap<String, String>,
    pub body: String,
}

lazy_static! {
    pub(crate) static ref METHOD_MAP: HashMap<&'static str, Method> = {
        let mut m = HashMap::new();
        m.insert("OPTIONS",Method::OPTIONS);
        m.insert("GET",Method::GET);
        m.insert("HEAD",Method::HEAD);
        m.insert("POST",Method::POST);
        m.insert("PUT",Method::PUT);
        m.insert("DELETE",Method::DELETE);
        m.insert("TRACE",Method::TRACE);
        m.insert("CONNECT",Method::CONNECT);
        m
    };
    pub(crate) static ref HTTP_VERSION_MAP: HashMap<&'static str, HTTPVersion> = {
        let mut m = HashMap::new();
        m.insert("1.0",HTTPVersion::Http10);
        m.insert("1.1",HTTPVersion::Http11);
        m.insert("2.0",HTTPVersion::Http20);
        m
    };
}
