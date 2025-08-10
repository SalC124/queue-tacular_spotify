use dioxus::prelude::*;

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
pub struct StatesLol {
    access_token: Signal<Option<String>>,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut states = use_context_provider(|| StatesLol {
        access_token: Signal::new(None),
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        Router::<Route> {}
    }
}
