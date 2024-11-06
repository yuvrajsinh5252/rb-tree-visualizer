use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use dioxus::prelude::*;

#[component]
pub fn Controls() -> Element {
    rsx! {
      div {
        class: "flex flex-col gap-4 border-2 items-center justify-center w-1/4 rounded-lg p-1",
        div {
          class: "flex gap-2",
          Input { value: "Enter..." }
          Button { value: "Insert" }
        }
        div {
          class: "flex gap-2",
          Input { value: "Enter..." }
          Button { value: "Delete" }
        }
        div {
          class: "flex justify-between mx-1 items-center",
          h3 { "Select Algorithm" }
          select {
            class: "w-1/2 border-2 p-1 rounded-md",
            option { "Red Black Tree" }
            option { "Binomial Heap" }
            option { "............." }
          }
        }
      }
    }
}
