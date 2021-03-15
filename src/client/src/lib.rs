// client
// src/lib.rs
//

#![allow(
    clippy::wildcard_imports,
)]

use seed::{prelude::*, *};

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

