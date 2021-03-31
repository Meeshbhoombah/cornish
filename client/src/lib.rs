#![allow(
    clippy::wildcard_imports,
)]

use seed::{prelude::*, *};

struct Window {
    connection: &'a &str,
}

fn init(_: Url) -> Window {
    // TODO establish real time connection with main server
    Window {
        connection: "Test"
    }
}

pub fn start() {
    App::builder(init)
        .build_and_start();
}
