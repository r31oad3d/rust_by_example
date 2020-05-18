use actix_web::{web, App, Either, Error, HttpResponse, HttpServer};
use bytes::Bytes;
use futures::future::ok;
use futures::stream::once;
use rand::prelude::*;

async fn index() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

fn index2() -> RegisterResult {
    if rand::random() {
        Either::A(HttpResponse::BadRequest().body("bad data"))
    } else {
        Either::B(Ok("Hello!"))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/async", web::to(index2)))
        .bind("127.0.0.1:8888")?
        .run()
        .await
}
