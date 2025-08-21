use crate::Route;
use dioxus::prelude::*;

use crate::AppStates;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Callback(code: String) -> Element {
    let mut app_states = use_context::<AppStates>();

    use_effect(use_reactive!(|code| {
        if !code.is_empty() {
            app_states.access_code.set(Some(code.clone()));
        }
    }));

    let stored_code = app_states.access_code.read().clone();

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            id: "callback",
            match stored_code.clone() {
                Some(v) => rsx! {
                    p { "code got!" }
                    p { "{v}" }
                },
                None => rsx! {
                    p { "you stink!" }
                },
            }
            Link {
                to: Route::Home {},
                match stored_code.clone() {
                    Some(_) => "finish auth and go home",
                    None => "retry auth and go home",
                }
            }
        }
    }
}
