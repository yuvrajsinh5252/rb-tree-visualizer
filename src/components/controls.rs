use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::store::CONTROLS;
use crate::store::RBTREE;
use crate::store::RED_BLACK_TREE;
use crate::store::SELECTED_TREE;
use crate::store::TREE_STATES;
use dioxus::prelude::*;

#[component]
pub fn Controls() -> Element {
    let mut addNode: Signal<i32> = use_signal(|| 0);
    let mut deleteNode: Signal<i32> = use_signal(|| 0);
    let mut disabled: Signal<bool> = use_signal(|| false);

    rsx! {
      div { class: "flex flex-col gap-6 bg-gradient-to-br from-slate-50 to-slate-100 max-sm:overflow-scroll max-sm:w-full shadow-lg rounded-xl p-6 w-1/4 relative border border-slate-200",

        div { class: "mb-4 flex justify-center items-center gap-2",
          i { class: "fas fa-tree text-blue-500 text-2xl" }
          h2 { class: "text-xl font-semibold text-slate-700", "Tree Controls" }
        }

        div { class: "flex flex-col gap-3 mt-2",
          label { class: "text-sm font-medium text-slate-600 flex items-center gap-2",
            i { class: "fas fa-code-branch text-blue-500" }
            "Select Algorithm"
          }
          div { class: "relative",
            select {
              class: "w-full px-4 py-2.5 bg-white border border-slate-300 rounded-lg shadow-sm
               appearance-none cursor-pointer
               hover:border-blue-400
               focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
               transition-all duration-200
               text-slate-700 font-medium",
              onchange: move |e| {
                  let selected_tree = SELECTED_TREE.read().clone();
                  match selected_tree.as_str() {
                      "Red Black Tree" => {
                          *RBTREE.write() = Default::default();
                      }
                      "Binomial Heap" => {}
                      _ => {}
                  }
                  *SELECTED_TREE.write() = e.value();
                  CONTROLS.write().ind.set(-1);
                  TREE_STATES.write().clear();
                  *RBTREE.write() = Default::default();
              },
              option {
                value: "",
                selected: true,
                disabled: true,
                class: "text-slate-400",
                "Choose an algorithm..."
              }
              option { class: "font-medium", "Red Black Tree" }
              option { class: "font-medium", "Binomial Heap" }
            }
            div { class: "absolute inset-y-0 right-0 flex items-center px-3 pointer-events-none",
              i { class: "fas fa-chevron-down text-slate-400" }
            }
          }
        }

        div { class: "flex flex-col gap-3",
          label { class: "text-sm font-medium text-slate-600", "Insert Node" }
          div { class: "flex gap-2",
            Input {
              value: *addNode.read(),
              placeholder: "Enter a value to insert...",
              oninput: move |value| {
                  addNode.set(value);
              },
            }
            Button {
              value: "Insert",
              color: Some(
                  if *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1 {
                      "bg-gray-400 cursor-not-allowed".to_string()
                  } else {
                      "bg-emerald-500 hover:bg-emerald-600 active:bg-emerald-700".to_string()
                  },
              ),
              onclick: move |_| {
                  let selected_tree = SELECTED_TREE.read().clone();
                  let node_val = *addNode.read();
                  match selected_tree.as_str() {
                      "Red Black Tree" => {
                          *disabled.write() = true;
                          spawn(async move {
                              RED_BLACK_TREE.write().insert(node_val).await;
                              *RBTREE.write() = RED_BLACK_TREE.read().clone();
                              *disabled.write() = false;
                          });
                      }
                      "Binomial Heap" => {}
                      _ => {}
                  }
                  addNode.set(0);
              },
              disabled: *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1
                  || *disabled.read(),
            }
          }
        }

        div { class: "flex flex-col gap-3",
          label { class: "text-sm font-medium text-slate-600", "Delete Node" }
          div { class: "flex gap-2",
            Input {
              value: *deleteNode.read(),
              placeholder: "Enter a value to delete...",
              oninput: move |value| {
                  deleteNode.set(value);
              },
            }
            Button {
              color: Some(
                  if *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1 {
                      "bg-gray-400 cursor-not-allowed".to_string()
                  } else {
                      "bg-red-500 hover:bg-red-600 active:bg-red-700".to_string()
                  },
              ),
              value: "Delete",
              onclick: move |_| {
                  let selected_tree = SELECTED_TREE.read().clone();
                  match selected_tree.as_str() {
                      "Red Black Tree" => {}
                      "Binomial Heap" => {}
                      _ => {}
                  }
                  deleteNode.set(0);
              },
              disabled: *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1,
            }
          }
        }

        div { class: "flex mt-auto flex-col gap-3 pt-6 border-t border-slate-300",
          label { class: "text-sm font-medium text-slate-600 flex justify-between items-center",
            span { class: "flex justify-center items-center gap-2",
              i { class: "fas fa-clock text-blue-500 mt-1" }
              "Animation Speed"
            }
            span { class: "px-2 py-1 bg-blue-100 text-blue-600 rounded-md text-xs font-semibold",
              "{CONTROLS.read().speed} ms"
            }
          }
          div { class: "relative",
            input {
              class: "w-full h-2 bg-blue-100 rounded-lg appearance-none cursor-pointer",
              style: "background: linear-gradient(to right, #3b82f6 0%, #3b82f6 var(--value), #e2e8f0 var(--value), #e2e8f0 100%);
               --value: {CONTROLS.read().speed}%",
              r#type: "range",
              min: "0",
              max: "100",
              value: CONTROLS.read().speed.read().to_string(),
              oninput: move |e| {
                  CONTROLS.write().speed.set(e.value().parse().unwrap_or(0));
              },
            }
            div { class: "absolute top-6 left-0 w-full flex justify-between text-xs text-slate-400",
              span { "Faster" }
              span { "Slower" }
            }
          }
        }
        div { class: "space-y-6 pt-4 border-slate-200",

          div { class: "grid grid-cols-3 gap-3",
            Button {
              value: "⟸ Prev",
              color: Some(
                  if *CONTROLS.read().ind.read() == 0 {
                      "bg-gray-400 cursor-not-allowed".to_string()
                  } else {
                      "bg-blue-500 hover:bg-blue-600 active:bg-blue-700".to_string()
                  },
              ),
              onclick: move |_| {
                  let curr_ind = *CONTROLS.read().ind.read();
                  if (curr_ind - 1) >= 0 && (curr_ind - 1) < TREE_STATES.read().len() as i32 {
                      let selected_tree = SELECTED_TREE.read().clone();
                      match selected_tree.as_str() {
                          "Red Black Tree" => {
                              *RBTREE.write() = TREE_STATES
                                  .read()[(curr_ind - 1) as usize]
                                  .clone();
                          }
                          "Binomial Heap" => {}
                          _ => {}
                      }
                      CONTROLS.write().ind.set(curr_ind - 1);
                  }
              },
              disabled: false,
            }
            Button {
              value: "Clear",
              color: Some("bg-slate-500 hover:bg-slate-600 active:bg-slate-700 w-full".to_string()),
              onclick: move |_| {
                  RBTREE.write().clear_tree();
                  TREE_STATES.write().clear();
                  CONTROLS.write().ind.set(-1);
              },
              disabled: *CONTROLS.read().ind.read() == -1,
            }
            Button {
              value: "Next ⟹",
              color: Some(
                  if *CONTROLS.read().ind.read() == TREE_STATES.read().len() as i32 - 1 {
                      "bg-gray-400 cursor-not-allowed".to_string()
                  } else {
                      "bg-blue-500 hover:bg-blue-600 active:bg-blue-700".to_string()
                  },
              ),
              onclick: move |_| {
                  let curr_ind = *CONTROLS.read().ind.read();
                  if (curr_ind + 1) < TREE_STATES.read().len() as i32 {
                      let selected_tree = SELECTED_TREE.read().clone();
                      match selected_tree.as_str() {
                          "Red Black Tree" => {
                              *RBTREE.write() = TREE_STATES
                                  .read()[(curr_ind + 1) as usize]
                                  .clone();
                          }
                          "Binomial Heap" => {}
                          _ => {}
                      }
                      CONTROLS.write().ind.set(curr_ind + 1);
                  }
              },
              disabled: *CONTROLS.read().ind.read() == TREE_STATES.read().len() as i32 - 1,
            }
          }
        }
      }
    }
}
