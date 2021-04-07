/// `connection.rs`
/// 
/// Takes in the `mediasoup` object bound to the browser's `window` object in
/// the `cornish-client`'s root `index.ts` and creates a useable Rust module 
/// such that a Webrtc connection can be made via WASM.
///

/// Reads in a connection string that speceficies the host, port, and,
/// optionally, path, at which a server connection has been made available to 
/// the client.
///
/// # Arguments
/// - `connection_url`: a String contatining the `HOST:PORT/PATH` to connect at
/// 
/// # Examples
/// ```
/// const CONNECTION_STRING = "127.0.0.1:3000/connect";
/// match let connection::new(CONNECTION_STRING) {
///     Ok(connection) => {},
///     Err(err) => {},
/// }
/// ```
pub fn new(connection_url: &str) {}

/*
struct Connection {
    
}
*/

