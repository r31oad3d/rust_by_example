#[allow(unused_imports)]
use {
    futures::{
        compat::Future01CompatExt,
        future::{FutureExt, TryFutureExt},
    },
    hyper::{
        rt::run, service::service_fn, Body, Client, Request, Response, Server,
        Uri,
    },
    log::{error, info},
    log4rs,
    std::net::SocketAddr,
};

fn init_logger() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

async fn serve_req(
    _req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    info!("request from {}", _req.uri());
    Ok(Response::new(Body::from("hello world!")))
}

async fn run_server(addr: SocketAddr) {
    info!("listening on http:{}", addr);
    let serve_future = Server::bind(&addr)
        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    if let Err(e) = serve_future.compat().await {
        error!("server error: {}", e);
    }
}

fn main() {
    init_logger();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();

    run(futures_01_future);
}
