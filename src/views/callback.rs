use crate::Route;
use dioxus::prelude::*;

use crate::StatesLol;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Callback(code: String) -> Element {
    let mut states_lol = use_context::<StatesLol>();

    use_effect(move || {
        let code_clone = code.clone();
        states_lol.access_token.set(Some(code_clone));
    });

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            id: "callback",
            h3 { "you're logged in!" }
            p { "{states_lol.access_token.read():?}" }

            Link {
                to: Route::Home {},
                "go home"
            }
        }
    }
}
