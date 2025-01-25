use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen", "404 Not Found" }
    }
}
