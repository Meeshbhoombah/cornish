#![allow(
    clippy::wildcard_imports,
)]

use seed::{prelude::*, *};
use web_sys::{
    RtcOfferOptions
};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
macro_rules! console_warn {
    ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

struct Client {
    connection: String,
}

// Events for
enum Msg {
    // Real-time Connection with `cornish-server`
    RtcEstablish(String),
    RtcEstablishSuccessful(String),  // TODO replace String w/ connection
    RtcEstablishFailed(String),      // TODO replace String w/ error
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Client {
    // TODO establish real time connection with main server
    Client {
        connection: "127.0.0.1:3000".to_string(),
    }
}

fn update(msg: Msg, model: &mut Client, _: &mut impl Orders<Msg>) {
    match msg {
        _ => {
            console_log!("Event");
        }
    }
}

fn view(client: &Client) -> Node<Msg> {
    h1![client.connection.clone()]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
