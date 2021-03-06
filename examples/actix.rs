extern crate actix_web;
#[macro_use]
extern crate rust_embed;
extern crate mime_guess;

use actix_web::body::Body;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use mime_guess::from_path;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => {
      let body: Body = match content {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
      };
      HttpResponse::Ok().content_type(from_path(path).first_or_octet_stream().as_ref()).body(body)
    }
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

fn index(_req: HttpRequest) -> HttpResponse {
  handle_embedded_file("index.html")
}

fn dist(req: HttpRequest) -> HttpResponse {
  let path = &req.path()["/dist/".len()..]; // trim the preceding `/dist/` in path
  handle_embedded_file(path)
}

fn main() {
  HttpServer::new(|| {
    App::new()
      .service(web::resource("/").route(web::get().to(index)))
      .service(web::resource("/dist/{_:.*}").route(web::get().to(dist)))
  })
  .bind("127.0.0.1:8000")
  .unwrap()
  .run()
  .unwrap();
}
