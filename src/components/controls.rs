use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::store::RED_BLACK_TREE;
use web_sys::console;

use dioxus::prelude::*;

#[component]
pub fn Controls() -> Element {
    let mut addNode: Signal<i32> = use_signal(|| 0);
    let mut deleteNode: Signal<i32> = use_signal(|| 0);

    rsx! {
      div {
        class: "flex flex-col gap-4 border-2 items-center justify-center w-1/4 rounded-lg p-1",
        div {
          class: "flex gap-2",
          Input {
            placeholder: "Enter...",
            oninput: move |value| {
              addNode.set(value);
            }
          }
          Button {
            value: "Insert",
            onclick: move |_| {
              RED_BLACK_TREE.write().insert(*addNode.read(), Default::default());
              addNode.set(0);
            },
            disabled: false
         }
        }
        div {
          class: "flex gap-2",
          Input {
            placeholder: "Enter...",
            oninput: move |value| {
              deleteNode.set(value);
            }
          }
          Button {
            value: "Delete",
            onclick: move |_| {
              RED_BLACK_TREE.write().delete(&(*deleteNode.read()));
              deleteNode.set(0);
            },
            disabled: false,
          }
        }
        div {
          class: "flex justify-between gap-8 mx-1 items-center",
          h3 { "Algorithm" }
          select {
            class: "w-full min-w-44 border-2 p-1 rounded-md",
            option { "Red Black Tree" }
            option { "Binomial Heap" }
          }
        }

        div {
          class: "flex gap-2",
          Button {
            value: "Print Tree",
            onclick: move |_| {
              let tree = RED_BLACK_TREE.read();
              let mut k = 0;
              for (key, value) in tree.into_iter() {
                  console::log_1(&format!("{:?} -> {:?}", key, value).into());
                  assert!(*key > k);
                  k = *key;
              }
            },
            disabled: false,
          }
        }
      }
    }
}
