#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (js_namespace = mediasoup)]
    type mediasoup;
}


fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    //log!(mediasoup);
    Model::default()
}


type Model = i32;


#[derive(Copy, Clone)]
enum Msg {
    Increment,
}


fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
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
