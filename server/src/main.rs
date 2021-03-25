use std::path::Path;
use std::io::Error as IoError;
use std::net::SocketAddr;

use futures::future;
use http::{header, StatusCode};
use http::response::Builder as ResponseBuilder;
use hyper::Server;
use hyper::{Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use hyper_staticfile::Static;

const DEFAULT_ADDRESS: &str = "127.0.0.1:3000";

async fn handle_request<B>(req: Request<B>, static_: Static) -> Result<Response<Body>, IoError> {
    if req.uri().path() == "/" {
        let res = ResponseBuilder::new()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header(header::LOCATION, "/hyper_staticfile/")
            .body(Body::empty())
            .expect("unable to build response");
        Ok(res)
    } else {
        static_.clone().serve(req).await
    }
}

#[tokio::main]
async fn main() {
    // todo rewrite with clap
    let a: SocketAddr = DEFAULT_ADDRESS
        .parse()
        .expect("Failed to parse address.");

    // todo update static file path
    let index = Static::new(Path::new("."));
    let make_service = make_service_fn(|_| {
        let client_pkg = index.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, client_pkg.clone())))
    });

    let server = Server::bind(&a).serve(make_service);
    println!("Static files available at http://{}/", &a);
    
    server.await
        .expect("Server failed");
}

