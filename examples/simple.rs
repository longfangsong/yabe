extern crate yabe;

use std::sync::Arc;

use yabe::http::request::Request;
use yabe::http::response::Response;
use yabe::http::response::Status;

fn handler(request: Request, response: &mut Response) {
    response.headers.insert("Content-Type".to_string(), "application/json".to_string());
    response.body = "{\"hello\":\"world\"}".to_string();
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

fn handler_redirect(request: Request, response: &mut Response) {
    response.status = Status::MovedPermanently;
    response.headers.insert("Location".to_string(), "http://127.0.0.1:8086/render?name=redirected".to_string());
}

fn handler_teapot(request: Request, response: &mut Response) {
    response.headers.insert("Content-Type".to_string(), "text/html".to_string());
    response.status = Status::ImATeapot;
    response.body = "<html><body><p>http 418: I'm a teapot</p><h5>欢迎各位来开源社区喝茶!</h5></body></html>".to_string();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = yabe::Server::new();
    server.add_handler("/", Arc::new(handler));
    server.add_handler("/html", Arc::new(handler_html));
    server.add_handler("/render", Arc::new(handler_render));
    server.add_handler("/redirect", Arc::new(handler_redirect));
    server.add_handler("/418", Arc::new(handler_teapot));
    server.add_handler("/teapot", Arc::new(handler_teapot));
    server.start_at("127.0.0.1:8086").await
}
