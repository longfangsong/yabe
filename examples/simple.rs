extern crate yabe;

use yabe::http::request::Request;
use yabe::http::response::Response;
use std::sync::Arc;

fn handler(request: Request, response: &mut Response) {
    response.body = "faq".to_string();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = yabe::Server::new();
    server.add_handler("/", Arc::new(handler));
    server.start_at("127.0.0.1:8086").await
}