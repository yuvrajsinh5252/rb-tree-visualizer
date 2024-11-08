use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::store::RED_BLACK_TREE;
use web_sys::console;

use dioxus::prelude::*;

#[component]
pub fn Controls() -> Element {
    rsx! {
      div {
        class: "flex flex-col gap-4 border-2 items-center justify-center w-1/4 rounded-lg p-1",
        div {
          class: "flex gap-2",
          Input {
            placeholder: "Enter...",
            // oninput: move |event| {
            //   console::log_1(&event.into());
            // }
         }
          Button {
            value: "Insert",
            onclick: move |_| {
              RED_BLACK_TREE.write().insert(1);
            },
            disabled: false
         }
        }
        // div {
          // class: "flex gap-2",
          // Input { value: "Enter..." }
          // Button {
          //   value: "Delete",
          //   onclick: move |_| {},
          //   disabled: false,
          // }
        // }
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

        div {
          class: "flex gap-2",
          Button {
            value: "Print Tree",
            onclick: move |_| {
              let tree = RED_BLACK_TREE.read();
              console::log_1(&format!("{:?}", tree.root).into());
            },
            disabled: false,
          }
        }
      }
    }
}
