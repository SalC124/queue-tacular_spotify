use dioxus::prelude::*;
use serde_json::Value;

use crate::{AppStates, Images, Song};

#[component]
pub fn Controls() -> Element {
    let token = use_context::<AppStates>().access_token.read().clone();
    let mut pause_code = use_signal(|| String::from("0"));
    let mut song_deets: Signal<Option<Song>> = use_signal(|| None);
    let mut play_code = use_signal(|| String::from("0"));
    let mut toggle_code = use_signal(|| String::from("0"));

    let pause = move |_: FormEvent| async move {
        pause_code.set(pause_playback().await);
    };

    let get_playback = move |_: FormEvent| async move {
        let deets = get_current_playback().await;

        match deets {
            None => {
                song_deets.set(None);
            }
            Some(ligma) => {
                song_deets.set(Some(ligma));
            }
        }
    };

    let play = move |_: FormEvent| async move {
        play_code.set(play(false, 0).await);
    };
    let toggle = move |_: FormEvent| async move {
        toggle_code.set(toggle_playback().await);
    };
    let get_queue = move |_: FormEvent| async move {
        get_queue().await;
    };

    match token {
        None => rsx! {},
        Some(_) => rsx! {
            form {
                onsubmit: pause,
                button { "pause {pause_code}" }
            }
            form {
                onsubmit: get_playback,
                button { "get playback" }
            }
            form {
                onsubmit: play,
                button { "play {play_code}" }
            }
            form {
                onsubmit: toggle,
                button { "toggle {toggle_code}" }
            }
            form {
                onsubmit: get_queue,
                button { "get queue" }
            }
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
                .unwrap()
                .status()
                .as_str()
                .to_string();
            status_code
        }
    }
}

pub async fn get_current_playback() -> Option<Song> {
    let mut app_states = use_context::<AppStates>();

    let access_token = app_states.access_token.read().clone();
    let song_index = app_states.song_index.read().clone();

    match access_token {
        None => None,
        Some(token) => {
            let deets = app_states
                .reqwest_client
                .read()
                .get("https://api.spotify.com/v1/me/player/currently-playing")
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();

            let sawng = Some(Song {
                index: song_index,
                id: {
                    let id = deets["item"]["id"].to_string();
                    id[1..id.len() - 1].to_string()
                },
                uri: {
                    let uri = deets["item"]["uri"].to_string();
                    uri[1..uri.len() - 1].to_string()
                },
                name: {
                    let name = deets["item"]["name"].to_string();
                    name[1..name.len() - 1].to_string()
                },
                artists: deets["item"]["artists"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|artist| artist["name"].as_str())
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>(),
                time_ms: deets["progress_ms"].as_i64().unwrap_or(0),
                is_playing: deets["is_playing"].as_bool().unwrap_or(false),
                images: {
                    let image_list: Vec<Value> = deets["item"]["album"]["images"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .to_vec();

                    let mut small_image = "".to_string();
                    let mut medium_image = "".to_string();
                    let mut large_image = "".to_string();

                    for i in image_list {
                        let url = i["url"].to_string();
                        let url = url[1..url.len() - 1].to_string();

                        match i["height"].as_i64() {
                            Some(640) => large_image = url.clone(),
                            Some(300) => medium_image = url.clone(),
                            Some(64) => small_image = url.clone(),
                            _ => {}
                        }
                        match i["width"].as_i64() {
                            Some(640) => large_image = url.clone(),
                            Some(300) => medium_image = url.clone(),
                            Some(64) => small_image = url.clone(),
                            _ => {}
                        }
                    }

                    if small_image == "" {
                        small_image =
                            "https://i.scdn.co/image/ab67616d000048512fd8f63fe08b94881bebe5f8"
                                .to_string();
                    }
                    if medium_image == "" {
                        medium_image =
                            "https://i.scdn.co/image/ab67616d00001e022fd8f63fe08b94881bebe5f8"
                                .to_string();
                    }
                    if large_image == "" {
                        large_image =
                            "https://i.scdn.co/image/ab67616d0000b2732fd8f63fe08b94881bebe5f8"
                                .to_string();
                    }

                    // .iter()
                    // .find(|img| img["height"].as_i64() == Some(300))
                    // .and_then(|img| img["url"].as_str())
                    // .unwrap_or("")
                    // .to_string();
                    Images {
                        small: small_image,
                        med: medium_image,
                        large: large_image,
                    }
                },
            });
            let index = app_states.song_index.read().clone() as usize;
            let mut song_vec = app_states.song_vector.write();

            if index < song_vec.len() {
                song_vec[index] = sawng.clone();
            } else {
                song_vec.push(sawng.clone());
            }
            // app_states
            //     .song_vector
            //     .insert(app_states.song_index.read().clone() as usize, song.clone());

            sawng
        }
    }
}

//PUT:
//     {
//     "uris":  ["spotify:track:3ZikLQCnH3SIswlGENBcKe","spotify:track:6I6Zjwlx4LSE1iojm94za1"], // this array can be used for the better-queue
//     "position_ms": 6000
// }

pub async fn play(absolute_time: bool, time_ms: i64) -> String {
    let app_states = use_context::<AppStates>();

    let access_token = app_states.access_token.read().clone();

    match access_token {
        None => "no token lol".to_string(),
        Some(token) => {
            let current_song_info = get_current_playback().await;

            match current_song_info {
                None => "idk bro".to_string(),
                Some(info) => {
                    let mut song_ms = info.time_ms;

                    match absolute_time {
                        true => {
                            song_ms = time_ms;
                        }
                        false => {
                            if time_ms == 0 {
                                // dont include `position_ms` (or ig you can ¯\_(ツ)_/¯)
                            } else {
                                song_ms += time_ms;
                            }
                        }
                    }
                    let uri = info.uri;

                    let body = serde_json::json!({
                        "uris": [uri],
                        "position_ms": song_ms
                    });

                    put_play(token, body).await
                }
            }
        }
    }
}

async fn put_play(token: String, body: Value) -> String {
    let app_states = use_context::<AppStates>();

    let status_code = app_states
        .reqwest_client
        .read()
        .put("https://api.spotify.com/v1/me/player/play")
        .header("Authorization", format!("Bearer {}", token))
        .json(&body)
        .send()
        .await
        .unwrap()
        .status()
        .as_str()
        .to_string();
    status_code
}

pub async fn toggle_playback() -> String {
    let token = use_context::<AppStates>().access_token.read().clone();
    match token {
        None => "no token".to_string(),
        Some(token) => {
            match get_current_playback().await {
                None => "bruh".to_string(),
                Some(song_deets) => {
                    match song_deets.is_playing {
                        false => {
                            put_play(token, serde_json::json!({ /* lol no */ })).await
                        }
                        true => pause_playback().await,
                    }
                }
            }
        }
    }
}

pub async fn get_queue() -> () {
    let mut app_states = use_context::<AppStates>();

    let access_token = app_states.access_token.read().clone();
    let mut song_vec = app_states.song_vector.write();
    let current_index = app_states.song_index.read().clone() as usize;

    match access_token {
        None => (),
        Some(token) => {
            let queue_array: Value = app_states
                .reqwest_client
                .read()
                .get("https://api.spotify.com/v1/me/player/queue")
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();

            let queue_array: Vec<Value> =
                queue_array["queue"].as_array().unwrap_or(&vec![]).to_vec();

            for (i, item) in queue_array.iter().enumerate() {
                let target_index = current_index + 1 + i;
                let song_item = Song {
                    index: target_index as i32,
                    id: {
                        let id = item["id"].to_string();
                        id[1..id.len() - 1].to_string()
                    },
                    uri: {
                        let uri = item["uri"].to_string();
                        uri[1..uri.len() - 1].to_string()
                    },
                    name: {
                        let name = item["name"].to_string();
                        name[1..name.len() - 1].to_string()
                    },
                    artists: item["artists"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter_map(|artist| artist["name"].as_str())
                        .map(|name| name.to_string())
                        .collect::<Vec<String>>(),
                    time_ms: 0,
                    is_playing: false,
                    images: {
                        let image_list: Vec<Value> = item["album"]["images"]
                            .as_array()
                            .unwrap_or(&vec![])
                            .to_vec();

                        let mut small_image = "".to_string();
                        let mut medium_image = "".to_string();
                        let mut large_image = "".to_string();

                        for i in image_list {
                            let url = i["url"].to_string();
                            let url = url[1..url.len() - 1].to_string();

                            match i["height"].as_i64() {
                                Some(640) => large_image = url.clone(),
                                Some(300) => medium_image = url.clone(),
                                Some(64) => small_image = url.clone(),
                                _ => {}
                            }
                            match i["width"].as_i64() {
                                Some(640) => large_image = url.clone(),
                                Some(300) => medium_image = url.clone(),
                                Some(64) => small_image = url.clone(),
                                _ => {}
                            }
                        }

                        if small_image == "" {
                            small_image =
                                "https://i.scdn.co/image/ab67616d000048512fd8f63fe08b94881bebe5f8"
                                    .to_string();
                        }
                        if medium_image == "" {
                            medium_image =
                                "https://i.scdn.co/image/ab67616d00001e022fd8f63fe08b94881bebe5f8"
                                    .to_string();
                        }
                        if large_image == "" {
                            large_image =
                                "https://i.scdn.co/image/ab67616d0000b2732fd8f63fe08b94881bebe5f8"
                                    .to_string();
                        }

                        Images {
                            small: small_image,
                            med: medium_image,
                            large: large_image,
                        }
                    },
                };

                if song_vec.len() <= song_item.index as usize {
                    song_vec.resize(target_index + 1, None);
                }
                song_vec[target_index] = Some(song_item.clone());
            }

            // if let Some(ref song) = sawng {
            //     let index = app_states.song_index.read().clone() as usize;
            //
            //     if index < song_vec.len() {
            //         song_vec[index] = song.clone();
            //     } else {
            //         song_vec.push(song.clone());
            //     }
            // }
        }
    }
}
