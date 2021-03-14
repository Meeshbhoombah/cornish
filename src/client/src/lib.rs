// client
// src/lib.rs
//
// This is the Desktop/Mobile browser client for the Architect x Cornish 
// system.
//
// It is the entry-point for end-Users into the Infrastructure adapted to
// Cornish.
//
//  + [ ] Checking if a Session is pre-existing
//      * If so, extract User info from session
//  + [ ] Establishing a socket connection with the Main Server
//      * If not, create a session with the User as Anon 
//          - TODO turn into proper authentication service
//  + [ ] Leveraging extracteed User token to download data
//  
//  Window system needs
//  - User data from Storage
//      + Are they logged in?
//      + What is their user token?
//      + What is the last block they received?
//  - Information about where to post User creation
//

#![allow(
    clippy::wildcard_imports,
)]

// mod Storage;

use seed::{prelude::*, *};

/*
struct Client {
    storage: Storage,
}

fn init(_: Url) -> Client {
    Client {
        storage: Storage::init(),
    }
}
*/

// fn after_mount() -> AfterMount<Client<'static>> {
}

pub fn start() {
    /*
    App::builder(init)
        .build_and_start();
    */
}

