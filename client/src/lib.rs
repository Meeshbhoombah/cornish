#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// mod connection;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    /*
    // replace with configuration from environment
    const SERVER_ADDRESS = "127.0.0.1:3000"

    // TODO send connection::Msg::Establish(SERVER_ADDRESS) after inital render to
    // create connection with the server
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
            // TODO take the seerver address and spawn a connection on a new thread
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
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
