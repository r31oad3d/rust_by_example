use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Client, Error, Request, Response, Server, Uri,
    },
    std::net::SocketAddr,
};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, Error> {
    println!("Got request at {:?}", _req.uri());
    let url_str = "http://www.rust-lang.org/en-US/";
    let url = url_str.parse::<Uri>().expect("failed to parse URL");

    let res = Client::new().get(url).await?;
    println!("request finished-- returning response");
    Ok(res)

    //Ok(Response::new(Body::from("hello world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listen on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, Error>(service_fn(serve_req))
    }));

    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    run_server(addr).await;
}
