// client
// src/lib.rs
//
// This is the Desktop/Mobile browser client for the Architect x Cornish 
// system.
//
// It is the entry-point for end-Users into the Infrastructure adapted to
// Cornish.
//

#![allow(
    clippy::wildcard_imports,
)]

use seed::{prelude::*, *};

use mod Session;

enum WindowRouter<'a> {
    Redirect(Session::User),
    //NotFound(None()),
    //Onboard(None),
    //Main(None()),
}

impl<'a> Default for WindowRouter<'a> {
    fn default() -> Self {
        WindowRouter::Redirect(Session::User)
    }
}

enum Msg {
    Route(),
}

pub fn start() {
    /*
    App::builder(update, view)
        .before_mount()
        .after_mount()
        .route(|url| Some(Msg::RouteChanged(url.try_into().ok())))
        .sink(sink)
        .build_and_start();
    */
}

