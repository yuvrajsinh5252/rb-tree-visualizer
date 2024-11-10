use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::store::CONTROLS;
use crate::store::RED_BLACK_TREE;
use crate::store::SELECTED_TREE;
use crate::store::TREE_STATES;

use dioxus::prelude::*;

#[component]
pub fn Controls() -> Element {
    let mut addNode: Signal<i32> = use_signal(|| 0);
    let mut deleteNode: Signal<i32> = use_signal(|| 0);

    rsx! {
      div {
        class: "flex flex-col gap-4 border-2 items-center justify-start w-1/4 rounded-lg p-3 pt-10",
        div {
          class: "flex gap-2",
          Input {
            value: *addNode.read(),
            placeholder: "Enter...",
            oninput: move |value| {
              addNode.set(value);
            }
          }
          Button {
            value: "Insert",
            color: Some(if *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1 {
              "bg-gray-500 cursor-not-allowed".to_string()
            } else {
              "bg-green-500/80 hover:bg-green-600/80".to_string()
            }),
            onclick: move |_| {
              let selected_tree = SELECTED_TREE.read().clone();

              match selected_tree.as_str() {
                "Red Black Tree" => {
                  RED_BLACK_TREE.write().insert(*addNode.read(), Default::default());
                }
                "Binomial Heap" => {
                  // Call Binomial Heap insertion function
                }
                _ => {}
              }

              TREE_STATES.write().push(RED_BLACK_TREE.read().clone());
              CONTROLS.write().ind.set(TREE_STATES.read().len() as i32 - 1);
              addNode.set(0);
            },
            disabled: *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1
         }
        }
        div {
          class: "flex gap-2",
          Input {
            value: *deleteNode.read(),
            placeholder: "Enter...",
            oninput: move |value| {
              deleteNode.set(value);
            }
          }
          Button {
            color: Some(if *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1{
              "bg-gray-500 cursor-not-allowed".to_string()
            } else {
              "bg-red-500/80 hover:bg-red-600/80".to_string()
            }),
            value: "Delete",
            onclick: move |_| {
              let selected_tree = SELECTED_TREE.read().clone();
              match selected_tree.as_str() {
                "Red Black Tree" => {
                  RED_BLACK_TREE.write().delete(&(*deleteNode.read()));
                }
                "Binomial Heap" => {
                  // Call Binomial Heap deletion function
                }
                _ => {}
              }

              TREE_STATES.write().push(RED_BLACK_TREE.read().clone());
              CONTROLS.write().ind.set(TREE_STATES.read().len() as i32 - 1);
              deleteNode.set(0);
            },
            disabled: *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1
          }
        }
        div {
          class: "flex justify-between gap-8 mx-1 items-center",
          h3 { "Algorithm" }
          select {
            class: "w-full min-w-44 border-2 p-1 rounded-md",
            onchange: move |e| {
              let selected_tree = SELECTED_TREE.read().clone();

              match selected_tree.as_str() {
                "Red Black Tree" => {
                  *RED_BLACK_TREE.write() = Default::default();
                }
                "Binomial Heap" => {
                  // Call Binomial Heap clear function
                }
                _ => {}
              }

              *SELECTED_TREE.write() = e.value();
              CONTROLS.write().ind.set(-1);
              TREE_STATES.write().clear();
              *RED_BLACK_TREE.write() = Default::default();
            },
            option { value: "", selected: true, disabled: true, "Select an Algorithm" }
            option { "Red Black Tree" }
            option { "Binomial Heap" }
          }
        }

        div {
          class: "flex flex-col justify-between gap-8 mx-1 p-10 items-center absolute bottom-0",
          div {
            class: "flex flex-col justify-center items-center gap-2 border-2 p-3 rounded-lg",
            div {
              class: "flex gap-4",
              input {
                class: "w-full min-w-44 border-2 p-1 rounded-md",
                r#type: "range",
                min: "0",
                max: "1000",
                value: CONTROLS.read().speed.read().to_string(),
                oninput: move |e| {
                    CONTROLS.write().speed.set(e.value().parse().unwrap_or(0));
                },
              }
              span {
                class: "text-center",
                "{CONTROLS.read().speed}"
              }
            }
            h2 { "Animation Speed" }
          }

          div {
            class: "flex gap-4",
            Button {
              value: "Next",
              onclick: move |_| {
                let curr_ind = *CONTROLS.read().ind.read();
                if (curr_ind + 1) < TREE_STATES.read().len() as i32 {
                  let selected_tree = SELECTED_TREE.read().clone();
                  match selected_tree.as_str() {
                    "Red Black Tree" => {
                      *RED_BLACK_TREE.write() = TREE_STATES.read()[(curr_ind + 1) as usize].clone();
                    }
                    "Binomial Heap" => {
                      // Call Binomial Heap next function
                    }
                    _ => {}
                  }

                  CONTROLS.write().ind.set(curr_ind + 1);
                }
              },
              disabled: false,
            }
            Button {
              value: "Clear",
              onclick: move |_| {
                RED_BLACK_TREE.write().clear_tree();
                TREE_STATES.write().clear();
                CONTROLS.write().ind.set(-1);
              },
              disabled: false,
            }
            Button {
              value: "Prev",
              onclick: move |_| {
                let curr_ind = *CONTROLS.read().ind.read();
                if (curr_ind - 1) >= 0 && (curr_ind - 1) < TREE_STATES.read().len() as i32 {
                  let selected_tree = SELECTED_TREE.read().clone();
                  match selected_tree.as_str() {
                    "Red Black Tree" => {
                      *RED_BLACK_TREE.write() = TREE_STATES.read()[(curr_ind - 1) as usize].clone();
                    }
                    "Binomial Heap" => {
                      // Call Binomial Heap prev function
                    }
                    _ => {}
                  }

                  CONTROLS.write().ind.set(curr_ind - 1);
                }
              },
              disabled: false,
            }
          }
        }
      }
    }
}
