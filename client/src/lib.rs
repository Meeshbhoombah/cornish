#![allow(clippy::wildcard_imports)]

use web_sys::{HtmlMediaElement};
use seed::{prelude::*, *};

// mod connection;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Client {
    /*
    // TODO change url based on environment (dev or prod)
    // dev, connection is hardcoded to 3000
    const SERVER_ADDRESS = "127.0.0.1:3000/ws"
    // prod = url + "/connect"

    // TODO send Msg::ConnectionEstablish(SERVER_ADDRESS) after inital render 
    */
    Client::default()
}

#[derive(Default)]
struct Client {
    send_display: ElRef<HtmlMediaElement>,
    receive_display: ElRef<HtmlMediaElement>,
}

#[derive(Copy, Clone)]
enum Msg {
    // ConnectionEstablish,
}

fn update(msg: Msg, model: &mut Client, _: &mut impl Orders<Msg>) {
    /*
    match msg {
        Msg::ConnectionEstablish(server_address) => {
            // TODO take the server address and spawn a connection on a new thread
            // TODO attach this connection to the client
            match let c = connection::new(server_address) {
                Ok(c)
                Err(e)
            }
        },
    }
    */
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Client) -> Node<Msg> {
    div![
        id!["container"],
        figure![
            video![
                el_ref(&model.send_display),
                id!["preview-send"],
                figcaption!["Send Preview"],
                attrs! {
                    At::Muted => true,
                    At::Controls => true,
                }
            ]
        ],
        figure![
            video![
                el_ref(&model.receive_display),
                id!["preview-receive"],
                figcaption!["Receive Preview"],
                attrs! {
                    At::Muted => true,
                    At::Controls => true,
                }
            ]
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
