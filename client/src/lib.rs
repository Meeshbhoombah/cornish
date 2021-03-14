#![allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
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

/*
pub fn start() {
    App::builder(update, view)
        .before_mount()
        .after_mount()
        .route(|url| Some(Msg::RouteChanged(url.try_into().ok())))
        .sink(sink)
        .build_and_start();
}
*/

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
