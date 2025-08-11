use dioxus::prelude::*;

use crate::{AppStates, Song};

#[component]
pub fn Controls() -> Element {
    let token = use_context::<AppStates>().access_token.read().clone();
    let mut status_code = use_signal(|| String::from("0"));
    let mut song_deets = use_signal(|| Song {
        index: 420,
        id: "poop".to_string(),
        uri: "poop".to_string(),
        name: "poop".to_string(),
        artists: vec!["poop".to_string(); 3],
        time_ms: 420,
        is_playing: false,
    });

    let pause = move |_: FormEvent| async move {
        status_code.set(pause_playback().await);
    };

    let get_playback = move |_: FormEvent| async move {
        song_deets.set(get_current_playback().await);
    };

    match token {
        None => rsx! {},
        Some(_) => rsx! {
            form {
                onsubmit: pause,
                button { "pause {status_code}" }
            }
            form {
                onsubmit: get_playback,
                button { "get playback" }
            }
            p { "{song_deets.read():?}" }
        },
    }
}

pub async fn pause_playback() -> String {
    let app_states = use_context::<AppStates>();

    let access_token = app_states.access_token.read().clone();

    match access_token {
        None => String::from("how?!"),
        Some(token) => {
            let status_code = app_states
                .reqwest_client
                .read()
                .put("https://api.spotify.com/v1/me/player/pause")
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .unwrap() // TODO
                .status()
                .as_str()
                .to_string();
            status_code
        }
    }
}

pub async fn get_current_playback() -> Song {
    let app_states = use_context::<AppStates>();

    let access_token = app_states.access_token.read().clone();
    let song_index = app_states.song_index.read().clone();

    match access_token {
        None => Song {
            index: 420,
            id: "poop".to_string(),
            uri: "poop".to_string(),
            name: "poop".to_string(),
            artists: vec!["poop".to_string(); 3],
            time_ms: 420,
            is_playing: false,
        },
        Some(token) => {
            let deets = app_states
                .reqwest_client
                .read()
                .get("https://api.spotify.com/v1/me/player/currently-playing")
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();
            Song {
                index: song_index,
                id: deets["item"]["id"].to_string(),
                uri: deets["item"]["uri"].to_string(),
                name: deets["item"]["name"].to_string(),
                artists: deets["item"]["artists"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|artist| artist["name"].as_str())
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>(),
                time_ms: deets["progress_ms"].as_i64().unwrap_or(0),
                is_playing: deets["is_playing"].as_bool().unwrap_or(false),
            }
        }
    }
}

// pub async play()
//
//PUT:
//     {
//     "uris":  ["spotify:track:3ZikLQCnH3SIswlGENBcKe","spotify:track:6I6Zjwlx4LSE1iojm94za1"], // this array can be used for the better-queue
//     "position_ms": 6000
// }
