use dioxus::prelude::*;

use crate::components::{canvas::Canvas, input::Input, navbar::Navbar};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col justify-end items-center h-screen",
            Navbar {}
            div {
                class: "flex h-[calc(100vh-3.5rem)] flex-row w-full p-1 z-50",
                Input {}
                Canvas {}
            }
        }
    }
}
