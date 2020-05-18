use actix_web::body::Body;
use actix_web::{
    web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

//Responder
impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn index() -> impl Responder {
    MyObj { name: "user" }
}
