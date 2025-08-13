use crate::{
    components::{Controls, Login, SongDisplay},
    AppStates,
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let access_token = use_context::<AppStates>().access_token.read().clone();
    match access_token {
        None => rsx! {
            Login {}
        },
        Some(_) => rsx! {
            SongDisplay {}
            Controls {}
        },
    }
}
