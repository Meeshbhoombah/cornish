use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};

const STATIC_FILES: &str = "../../client/";
const DEFAULT_ADDRESS: &str = "127.0.0.1:3000";

async fn handle_connection(conn_stream: TcpStream, conn_addr: SocketAddr) {
    println!("{:?} {:?}", conn_stream, conn_addr) 
}

#[tokio::main]
async fn main() {
    // TODO rewrite with clap
    let bind_socket = TcpListener::bind(DEFAULT_ADDRESS).await;
    let listener = bind_socket.expect("Unable to bind to socket.");
    println!("Listening...");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
}

