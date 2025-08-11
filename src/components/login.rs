use dioxus::prelude::*;

use std::collections::HashMap;
use std::env;
use url::Url;

use crate::{AppStates};

const CLIENT_ID: &str = "432051cfd6a446d9b3adfeea29af2748";

#[component]
pub fn Login() -> Element {
    let mut app_states = use_context::<AppStates>();

    let port = env!("PORT");
    let client_secret = env!("CLIENT_SECRET");

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

    let access_code = app_states.access_code.read().clone();
    let access_token = app_states.access_token.read().clone();

    match access_code {
        None => {
            return rsx! {
                h2 { "log in "
                    a { href: auth_url.as_str(), "here" }
                }
            }
        }
        Some(code) => match access_token {
            None => {
                spawn(async move {
                    // Create form data
                    let mut form_data = HashMap::new();
                    form_data.insert("grant_type", "authorization_code");
                    form_data.insert("code", &code);
                    form_data.insert("redirect_uri", &redirect_uri);

                    // Create basic auth header
                    let credentials = format!("{}:{}", CLIENT_ID, client_secret);
                    let encoded_credentials = base64::encode(credentials);
                    let auth_header = format!("Basic {}", encoded_credentials);

                    let response = app_states.reqwest_client.read()
                        .post("https://accounts.spotify.com/api/token")
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .header("Authorization", auth_header)
                        .form(&form_data)
                        .send()
                        .await
                        .unwrap() // TODO
                        .json::<serde_json::Value>()
                        .await
                        .unwrap(); // TODO

                    let token = response["access_token"].clone().to_string();
                    let token = &token[1..token.len() - 1];

                    app_states.access_token.set(Some(token.to_string()));
                });
                return rsx! {
                    h2 { "you still need to get your token" }
                };
            }
            Some(access_token) => {
                return rsx! {
                    h2 { "youre logged in! :D" }
                    p { "{access_token}" }
                };
            }
        },
    };
}
