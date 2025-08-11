use dioxus::prelude::*;

use reqwest::Client;
use views::{Callback, Home};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
        #[route("/")]
        Home {},
        #[route("/callback?:code")]
        Callback { code: String },
}

#[derive(Clone, Debug)]
pub struct AppStates {
    access_code: Signal<Option<String>>,
    access_token: Signal<Option<String>>,
    song_index: Signal<i32>,
    song_vector: Signal<Vec<Song>>,
    reqwest_client: Signal<reqwest::Client>,
}

#[derive(Clone, Debug)]
pub struct Song {
    index: i32,
    id: String,
    uri: String,
    name: String,
    artists: Vec<String>,
    time_ms: i64,
    is_playing: bool,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _: AppStates = use_context_provider(|| AppStates {
        access_code: Signal::new(None),
        access_token: Signal::new(None),
        song_index: Signal::new(0),
        song_vector: Signal::new(vec![]),
        reqwest_client: Signal::new(Client::new()),
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        Router::<Route> {}
    }
}
