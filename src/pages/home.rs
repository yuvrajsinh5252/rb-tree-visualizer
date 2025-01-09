// use crate::store::COUNT;
use dioxus::prelude::*;

use crate::components::{canvas::Canvas, controls::Controls};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col justify-end items-center h-screen",
            div {
                class: "flex h-[calc(100vh-3.5rem)] flex-row w-full p-1 z-50 gap-1",
                Controls {}
                Canvas {}
            }
        }
    }
}
