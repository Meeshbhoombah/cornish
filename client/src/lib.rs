#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// mod connection;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    /*
    // TODO change url based on environment (dev or prod)
    // dev, connection is hardcoded to 3000
    const SERVER_ADDRESS = "127.0.0.1:3000"
    // prod = url + "/connect"

    // TODO send connection::Msg::Establish(SERVER_ADDRESS) after inital render 
    */

    Model::default()
}

type Model = i32;

/*
struct Client {

}
*/

#[derive(Copy, Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }

    /*
    match msg {
        connection::Msg::Establish => {
            // TODO take the server address and spawn a connection on a new thread
            // TODO attach this connection to the client
            match let c = connection::new(SERVER_ADDRESS) {
                Ok(c)
                Err(e)
            }
        }
    }
    */
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> Node<Msg> {
    div![
        id!["container"],
        figure![
            video![
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
