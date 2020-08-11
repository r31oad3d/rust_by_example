use actix_study::module4::{config, scoped_config};
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}

/*
/         -> "/"
/app      -> "app"
/api/test -> "test"
*/
