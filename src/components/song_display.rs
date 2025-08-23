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
            let song_index = app_states.song_index.read().clone();
            let current_song = &song_vec[song_index as usize];

            return rsx! {
                div { id: "queue",
                    for song_item in song_vec.clone() {
                        div {
                            key: "{song_item.index}",
                            style: r#"
                                display: inline-block;
                                "#,
                            img { src: "{song_item.images.small}" }
                            p {

                                style: if song_index == song_item.index {
                                    r#"
                                        color: red;
                                    "#
                                },
                                "{song_item.index}" }
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
