use dioxus::prelude::*;

use crate::{components::controls::*, AppStates, Song};

#[component]
pub fn SongDisplay() -> Element {
    // let token = use_context::<AppStates>().access_token.read().clone();
    // let mut pause_code = use_signal(|| String::from("0"));
    // let mut song_deets: Signal<Option<Song>> = use_signal(|| None);
    // let mut play_code = use_signal(|| String::from("0"));
    //
    // let pause = move |_: FormEvent| async move {
    //     pause_code.set(pause_playback().await);
    // };
    rsx! {
        p { "hi" }
    }
}
