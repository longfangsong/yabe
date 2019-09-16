#[macro_use]
extern crate lazy_static;

use tokio::net::TcpListener;
use tokio::prelude::*;
use crate::http::request::Request;
use crate::http::response::{Response, Status};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

mod parser;
pub mod http;

pub type Handler = dyn Fn(Request, &mut Response) + Send + Sync + 'static;

pub struct Server {
    handlers: HashMap<&'static str, Arc<Handler>>
}


impl Server {
    pub fn new() -> Self {
        Server {
            handlers: HashMap::new()
        }
    }
    pub fn add_handler(&mut self, uri: &'static str, handler: Arc<Handler>) {
        self.handlers.insert(uri, handler.clone());
    }
    pub async fn start_at(self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut listener = TcpListener::bind(address).await?;
        loop {
            let (mut socket, _) = listener.accept().await?;
            let handlers = self.handlers.clone();
            tokio::spawn(async move {
                let mut buffer = [0; 4096];
                socket.read(&mut buffer).await;
                let result = parser::parse(std::str::from_utf8(&buffer).unwrap());
                let request = result.unwrap().1;
                let mut response = Response::new();
                let uri = request.uri.path.clone();
                let handler = handlers.get(uri.as_str());
                if let Some(handler) = handler {
                    handler(request, &mut response);
                } else {
                    response.status = Status::BadRequest;
                }
                socket.write(response.marshal().as_bytes()).await;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Server;

    #[tokio::test]
    async fn it_works() {
        let future = Server::new().start_at("127.0.0.1:8000");
        future.await;
    }
}
