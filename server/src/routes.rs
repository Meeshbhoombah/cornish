use std::io::Error as IoError;
use std::net::SocketAddr;

use tokio::net::{TcpStream};
use http::response::Builder as ResponseBuilder;
use http::{header, StatusCode};
use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response};
use hyper_staticfile::Static;

pub async fn connect(conn_stream: TcpStream, conn_addr: SocketAddr) {
    println!("{:?} {:?}", conn_stream, conn_addr)
}

pub async fn handle<B>(req: Request<B>, static_: Static) -> Result<Response<Body>, IoError> {
    match req.uri().path() {
        "/connect" => {
            let res = ResponseBuilder::new()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header(header::LOCATION, "/index.html")
                .body(Body::empty())
                .expect("unable to build response");
            Ok(res)
        },
        _ => {
            static_.serve(req).await
        }
    }
}
