use std::io::Error as IoError;

use http::{header, StatusCode};
use http::response::Builder as ResponseBuilder;
use hyper::{Body, Request, Response};
use hyper_staticfile::Static;

pub async fn handle<B>(req: Request<B>, static_: Static) -> Result<Response<Body>, IoError> {

    /*
    match req.uri().path {
        "/pgk/" => {
            static_.clone().serve(req.await)
        },
        "/connect" => {
            println!("TCP Connection");
        },
        _ => {
            static_.clone().serve(req.await)
        }
    }
    */


    if req.uri().path() == "/" {
        let res = ResponseBuilder::new()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header(header::LOCATION, "/index.html")
            .body(Body::empty())
            .expect("unable to build response");
        Ok(res)
    } else {
        static_.clone().serve(req).await
    }
}
