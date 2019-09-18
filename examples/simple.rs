extern crate yabe;

use yabe::http::request::Request;
use yabe::http::response::Response;
use std::sync::Arc;

fn handler(request: Request, response: &mut Response) {
    response.headers.insert("Content-Type".to_string(), "application/json".to_string());
    response.body = "{\"insult\":\"faq\"}".to_string();
}

fn handler_html(request: Request, response: &mut Response) {
    response.headers.insert("Content-Type".to_string(), "text/html".to_string());
    response.body = "<!doctype html><html lang=\"zh-cn\"><body><h1>My First Heading</h1><h2>Smaller Heading</h2><h4>And even smaller</h4><h4>Smallest</h4><p>My first paragraph.</p>\
    <a href=\"http://127.0.0.1:8086/render?name=abc\">I'm abc</a>\
    <a href=\"http://127.0.0.1:8086/render?name=def\">I'm def</a>\
    </body></html>".to_string();
}

fn handler_render(request: Request, response: &mut Response) {
    response.headers.insert("Content-Type".to_string(), "text/html".to_string());
    response.body = format!("<html><body><h1>{}'s First Heading</h1><p>{}'s first paragraph.</p></body></html>",
                            request.uri.query.get("name").unwrap(), request.uri.query.get("name").unwrap()
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = yabe::Server::new();
    server.add_handler("/", Arc::new(handler));
    server.add_handler("/html", Arc::new(handler_html));
    server.add_handler("/render", Arc::new(handler_render));
    server.start_at("127.0.0.1:8086").await
}
