use crate::Route;
use dioxus::prelude::*;

use crate::AppStates;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Callback(code: String) -> Element {
    let mut app_states = use_context::<AppStates>();

    use_effect(move || {
        if code != "" {
            let code_clone = code.clone();
            app_states.access_code.set(Some(code_clone));
        }
    });

    let code = app_states.access_code.read().clone();

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            id: "callback",
            match code.clone() {
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
                match code.clone() {
                    Some(_) => "finish auth and go home",
                    None => "retry auth and go home",
                }
            }
        }
    }
}
