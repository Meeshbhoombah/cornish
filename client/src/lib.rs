#![allow(clippy::wildcard_imports)]

use serde::{Serialize, Deserialize};
use web_sys:: {
    HtmlMediaElement,
    MediaStream,
    WebSocket,
};

use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Client {
    const SERVER_ADDRESS: &str = "ws://localhost:3000/ws";

    Client {
        connection: WebSocket::new(SERVER_ADDRESS).expect("`new()` connection"),
        send_display: ElRef::<HtmlMediaElement>::default(),
        send_stream: MediaStream::new().expect("`new()` send_stream"),
        receive_display: ElRef::<HtmlMediaElement>::default(),
        receive_stream: MediaStream::new().expect("`new()` receive_stream"), 
    }
}


struct Client {
    connection: WebSocket,
    send_display: ElRef<HtmlMediaElement>,
    send_stream: MediaStream,
    receive_display: ElRef<HtmlMediaElement>,
    receive_stream: MediaStream,
}


#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
enum ClientMessage {
    Init,
    
    ConnectProducerTransport,
    Produce,

    ConnectConsumerTransport,
    Consume,

    Resume
}

#[derive(Serialize, Deserialize)]
struct ClientInit {
    action: String,
}

#[derive(Copy, Clone)]
enum Msg {
    SendDisplayMetaDataLoaded,
    ReceiveDisplayMetaDataLoaded,

    Send(ClientMessage),
}

fn update(msg: Msg, model: &mut Client, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendDisplayMetaDataLoaded => {
            let video = model.send_display.get().expect("`get()` send_display");
            video.play().expect("`play()` send_display");
        },
        Msg::ReceiveDisplayMetaDataLoaded => {
            let video = model.receive_display.get().expect("`get()` recieve_display");
            video.play().expect("`play()` receive_display");
        },
        Msg::Send(message) => {
            match message {
                ClientMessage::Init => {
                    let msg = ClientInit {
                        action: String::from("Init"),
                    };

                    let data = JsValue::from_serde(&msg).unwrap();
                },
                _ => {},
            }
        },
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Client) -> Node<Msg> {
    div![
        id!["container"],
        figure![
            video![
                el_ref(&model.send_display),
                ev(Ev::LoadedMetaData, |_| Msg::SendDisplayMetaDataLoaded),
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
                ev(Ev::LoadedMetaData, |_| Msg::ReceiveDisplayMetaDataLoaded),
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
