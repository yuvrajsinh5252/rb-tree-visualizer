use crate::{components::navbar::Navbar, Route};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "flex flex-col h-screen bg-zinc-50",
            Navbar {}
            div { class: "flex flex-col flex-1", Router::<Route> {} }
        }
    }
}
