use crate::components::button::Button;
use dioxus::prelude::*;

#[component]
pub fn Input() -> Element {
    rsx! {
      div {
        class: "flex flex-col border-2 items-center justify-center w-1/4",
        div {
          class: "flex flex-col",
          input {
            r#type: "text",
            placeholder: "Enter node value...",
          }
          Button { value: "Add Node" }
        }
        div {
          class: "flex flex-col",
          input {
            r#type: "text",
            placeholder: "delete node...",
          }
          Button { value: "Delete Node" }
        }
        select {
          option { "Option 1" }
          option { "Option 2" }
          option { "Option 3" }
        }
      }
    }
}
