use std::net::SocketAddr;
use std::io::Error as IoError;

use tokio::net::{TcpListener, TcpStream};

use http::response::Builder as ResponseBuilder;
use http::{header, StatusCode};
use hyper::{Body, Request, Response};
use hyper_staticfile::Static;
use tokio_tungstenite::WebSocketStream;

use hyper::server::conn::AddrStream;
use hyper::upgrade::{Upgraded, OnUpgrade};

async fn websocket_handle_events(
    _ip: SocketAddr,
    ws_stream: WebSocketStream<Upgraded>
) -> Result<(), ()> {
    println!("Fired");

    Ok(())
}

pub async fn handle<B>(
    ip: SocketAddr, 
    req: Request<B>,
    static_: Static
) -> Result<Response<Body>, IoError> {
    match req.uri().path() {
        "/connect" => {
            tokio::spawn(async move {
                match hyper::upgrade::on(&mut req).await {
                    Ok(upgraded) => {
                        let ws_stream = WebSocketStream::from_raw_socket(
                            upgraded,
                            tokio_tungstenite::tungstenite::protocol::Role::Server,
                            None,
                        ).await;

                        if let Err(e) = websocket_handle_events(ip, ws_stream).await {
                            eprintln!("server foobar io error")
                        };
                    }
                    Err(e) => eprintln!("upgrade error: {}", e),
                }
            });            

            static_.serve(req).await
        },
        _ => {
            static_.serve(req).await
        }
    }
}
