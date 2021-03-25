// client
// src/lib.rs
//

#![allow(
    clippy::wildcard_imports,
)]

use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

type Model = i32;

enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

/*
struct Client {
}

fn init(_: Url) -> Client {
    // TODO establish real time connection with main server
}

struct Msg {
    SignIn(User),
    LogOut(User),
}

pub fn start() {
    /*
    App::builder(init)
        .build_and_start();
    */
}
*/
