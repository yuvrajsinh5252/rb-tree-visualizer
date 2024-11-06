use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    rsx! {
        div {
          class: "flex flex-col border-2  items-center justify-center w-3/4 rounded-lg",
          "This is the canvas"
        }
    }
}
