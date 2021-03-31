#![allow(
    clippy::wildcard_imports,
)]

use seed::{prelude::*, *};

const CONNECTION_STRING: &str = "";

struct Window {
    connection: String,
}

enum Msg {
    UplinkEstablish(String),
    UplinkEstablishSuccessful(String),
    UplinkEstablishFailed(String),
    DownlinkEstablish(String),
    DownlinkEstablishSuccessful(String),
    DownlinkEstablishFailed(String),
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Window {
    // TODO establish real time connection with main server
    orders.notify(Msg::UplinkEstablish);

    Window {
        connection: "Test".to_string(),
    }
}

fn update(msg: Msg, model: &mut Window, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UplinkEstablish(connection_string) => {
            println!("Establishing uplink...")
        },
        Msg::UplinkEstablishSuccessful(connection_string) => {
        },
        Msg::UplinkEstablishFailed(connection_string) => {
        },
        Msg::DownlinkEstablish(connection_string) => {
        },
        Msg::DownlinkEstablishSuccessful(connection_string) => {
        },
        Msg::DownlinkEstablishFailed(connection_string) => {
        }
    }
}

fn view(model: &Window) -> Node<Msg> {
    h1!["Hello, World!"]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
