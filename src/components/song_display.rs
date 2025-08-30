use std::cmp::Ordering;

use dioxus::prelude::*;

use crate::{components::controls::*, AppStates, Song};

#[component]
pub fn SongDisplay() -> Element {
    let app_states = use_context::<AppStates>();
    let token = app_states.access_token.read().clone();
    let song_vec = app_states.song_vector.read();

    match song_vec.len().cmp(&0) {
        Ordering::Greater => {
            let song_index = *app_states.song_index.read();
            let current_song = &song_vec[song_index as usize];

            return rsx! {
                div { id: "queue",
                    for song_item in song_vec.clone().into_iter().flatten() {
                        div {
                            key: "{song_item.index}",
                            style: r#"
                                background-color: #444;
                                color: white;
                                padding: 12px 24px;
                                border-radius: 6px;
                                cursor: pointer;
                            "#,
                            img { src: "{song_item.images.small}" }
                            p {
                                style: if song_index == song_item.index {
                                    r#"
                                        color: red;
                                    "#
                                },
                                "{song_item.index} | {song_item.name}"
                            }
                        }
                    }
                }
            };
        }
        _ => rsx! {
            p { "get playback, you playbackless FREAK!" }
        },
    }
}
