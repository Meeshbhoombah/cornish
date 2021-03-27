use std::path::Path;
use std::net::SocketAddr;

use futures::future;
use tokio::net::{TcpListener, TcpStream};
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper_staticfile::Static;

mod routes;

const CLIENT: &str = "../client/";
const DEFAULT_ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let addr: SocketAddr = DEFAULT_ADDRESS
        .parse()
        .expect("Unable to parse address.");

    
    let listener = TcpListener::bind(&addr).await
        .expect("Unable to bind to provided address.");
    
    println!("Listening at: {}", &addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(routes::connect(stream, addr));
    }

    /*
    let client = Static::new(Path::new(CLIENT));
    let cornish = make_service_fn(|_| {
        // make a reference to client available inside of the `service_fn`
        // using `clone` rather than `move` to only copy the ref to `client`
        let client = client.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| routes::handle(req, client.clone())))
    });
    
    let server = Server::bind(&addr).serve(cornish);
    println!("Static files available at http://{}/", &addr);
    
    server.await
        .expect("Server failed");
    */
}

