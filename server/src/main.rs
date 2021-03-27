use std::net::SocketAddr;
use std::path::Path;

use futures::future;
use hyper::{Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper_staticfile::Static;

mod routes;

const CLIENT: &str = "../client/";
const DEFAULT_ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let addr: SocketAddr = DEFAULT_ADDRESS.parse().expect("Unable to parse address.");

    // Does Path::new make a syscall? Is it unwise to use it for each connection?
    let cornish = make_service_fn(|_| {
        let client = Static::new(Path::new(CLIENT));

        future::ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
            routes::handle(req, client.clone())
        }))       
    });

    let server = Server::bind(&addr).serve(cornish);
    println!("Listening at http://{}/", &addr);

    if let Err(err) = server.await {
        eprintln!("{}", err);
    }
}
