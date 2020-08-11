use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("hello {}!", app_name)
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn _index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/counter", web::get().to(_index))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
