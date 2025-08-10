use std::env;

use dioxus::logger;
use dioxus::prelude::*;
use url::Url;

use crate::StatesLol;

const CLIENT_ID: &str = "432051cfd6a446d9b3adfeea29af2748";

#[component]
pub fn Login() -> Element {
    let states_lol = use_context::<StatesLol>();

    let port = env!("PORT");
    let redirect_uri = format!("http://127.0.0.1:{}/callback", port);

    let scope =
        "user-read-private user-read-email user-modify-playback-state user-read-playback-state";

    let mut auth_url = Url::parse("https://accounts.spotify.com/authorize").unwrap();
    auth_url
        .query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", CLIENT_ID)
        .append_pair("scope", scope)
        .append_pair("redirect_uri", &redirect_uri);

    match *states_lol.access_token.read() {
        None => {
            return rsx! {
                h2 { "log in "
                    a { href: auth_url.as_str(), "here" }
                }
            }
        }
        Some(_) => {
            return rsx! {
                h2 { "youre logged in, buster brown" }
            }
        }
    };
}
