use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::store::CONTROLS;
use crate::store::RBTREE;
use crate::store::SELECTED_TREE;
use crate::store::TREE_STATES;
use dioxus::prelude::*;

#[component]
pub fn Controls() -> Element {
    let mut addNode: Signal<i32> = use_signal(|| 0);
    let mut deleteNode: Signal<i32> = use_signal(|| 0);
    let mut disabled: Signal<bool> = use_signal(|| false);

    rsx! {
<<<<<<< HEAD
      div {
        class: "max-sm:overflow-scroll flex flex-col gap-6 border-2 border-gray-200 shadow-lg items-center justify-between w-[20%] rounded-xl p-6 bg-white",

        div {
          class: "flex flex-col justify-start gap-6",
          div {
            class: "w-full space-y-2",
            h3 {
              class: "text-lg font-semibold text-gray-700",
              "Select Algorithm"
            }
            select {
              class: "w-full border-2 p-2 rounded-lg bg-gray-50 hover:bg-gray-100 transition-colors focus:outline-none focus:border-blue-500",
              onchange: move |e| {
                let selected_tree = SELECTED_TREE.read().clone();

                match selected_tree.as_str() {
                  "Red Black Tree" => {
                    *RBTREE.write() = Default::default();
                  }
                  "Binomial Heap" => {
                    // Call Binomial Heap clear function
                  }
                  _ => {}
                }

                *SELECTED_TREE.write() = e.value();
                CONTROLS.write().ind.set(-1);
                TREE_STATES.write().clear();
                *RBTREE.write() = Default::default();
              },
              select {  }
              option { value: "", selected: true, disabled: true, "Select an Algorithm" }
              option { "Red Black Tree" }
              option { "Binomial Heap" }
            }
          }

          div {
            class: "w-full space-y-4",
            div {
              class: "flex gap-3",
              Input {
                value: *addNode.read(),
                placeholder: "Enter value to insert...",
                oninput: move |value| { addNode.set(value); }
              }
              Button {
                value: "Insert",
                color: Some(if *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1 {
                  "bg-gray-400 cursor-not-allowed".to_string()
                } else {
                  "bg-emerald-500 hover:bg-emerald-600 text-white shadow-md".to_string()
                }),
                disabled: *disabled.read(),
                onclick: move |_| {
                  let selected_tree = SELECTED_TREE.read().clone();
                  let node_val = *addNode.read();

                  match selected_tree.as_str() {
                    "Red Black Tree" => {
                      let mut current_tree = RBTREE.read().clone();
                      current_tree.insert(node_val);
                      *RBTREE.write() = current_tree;
                    }
                    "Binomial Heap" => {
                      // Call Binomial Heap insertion function
                    }
                    _ => {}
                  }

                  TREE_STATES.write().push(RBTREE.read().clone());
                  CONTROLS.write().ind.set(TREE_STATES.read().len() as i32 - 1);
                  addNode.set(0);
                }
              }
            }

            div {
              class: "flex gap-3",
              Input {
                value: *deleteNode.read(),
                placeholder: "Enter value to delete...",
                oninput: move |value| { deleteNode.set(value); }
              }
              Button {
                value: "Delete",
                color: Some(if *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1 {
                  "bg-gray-400 cursor-not-allowed".to_string()
                } else {
                  "bg-red-500 hover:bg-red-600 text-white shadow-md".to_string()
                }),
                onclick: move |_| {
                  let selected_tree = SELECTED_TREE.read().clone();
                  let delete_val = *deleteNode.read();

                  match selected_tree.as_str() {
                    "Red Black Tree" => {
                      let mut current_tree = RBTREE.read().clone();
                      current_tree.delete(delete_val);

                      // Update the trees after modification
                      *RBTREE.write() = current_tree.clone();
                      *RBTREE.write() = current_tree;
                    }
                    "Binomial Heap" => {
                      // Call Binomial Heap deletion function
                    }
                    _ => {}
                  }

                  TREE_STATES.write().push(RBTREE.read().clone());
                  CONTROLS.write().ind.set(TREE_STATES.read().len() as i32 - 1);
                  deleteNode.set(0);
                }
              }
            }
          }
        }

        div {
          class: "w-full space-y-4 mt-4",
          h3 {
            class: "text-lg font-semibold text-gray-700 text-center",
            "Animation Controls"
          }

          div {
            class: "flex flex-col gap-2 p-4 bg-gray-50 rounded-lg",
            div {
              class: "flex items-center gap-4",
              input {
                class: "flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer",
                r#type: "range",
                min: "0",
                max: "100",
                value: CONTROLS.read().speed.read().to_string(),
                oninput: move |e| {
                    CONTROLS.write().speed.set(e.value().parse().unwrap_or(0));
                },
              }
              span {
                class: "text-sm font-medium text-gray-600 min-w-[3rem] text-right",
                "{CONTROLS.read().speed}"
              }
            }
            p {
              class: "text-sm text-gray-600 text-center",
              "Animation Speed"
            }
          }

          div {
            class: "flex justify-center gap-4 mt-4",
            Button {
              value: "← Prev",
              color: Some("bg-blue-500 hover:bg-blue-600 text-white shadow-md".to_string()),
              onclick: move |_| {
                let curr_ind = *CONTROLS.read().ind.read();
                if (curr_ind - 1) >= 0 && (curr_ind - 1) < TREE_STATES.read().len() as i32 {
                  let selected_tree = SELECTED_TREE.read().clone();
                  match selected_tree.as_str() {
                    "Red Black Tree" => {
                      *RBTREE.write() = TREE_STATES.read()[(curr_ind - 1) as usize].clone();
                    }
                    "Binomial Heap" => {
                      // Call Binomial Heap prev function
                    }
                    _ => {}
=======
        div {
            class: "flex flex-col gap-6 border-2 border-gray-200/50 bg-white/30 backdrop-blur-md items-center justify-start w-1/4 rounded-xl p-6 shadow-lg max-sm:w-full max-sm:overflow-scroll",

            div {
              class: "w-full space-y-3",
              h3 {
                class: "text-lg font-semibold text-gray-700 flex items-center gap-2",
                i { class: "fas fa-code-branch" }
                "Algorithm"
              }
              div {
                class: "relative",
                select {
                  class: "w-full border-2 p-3 rounded-lg bg-white/70 backdrop-blur-sm
                       text-gray-700 font-medium cursor-pointer appearance-none
                       transition-all duration-200 hover:border-blue-400
                       focus:border-blue-500 focus:ring-2 focus:ring-blue-200
                       shadow-sm hover:shadow-md",
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
                  option {
                    value: "",
                    selected: true,
                    disabled: true,
                    "Select an Algorithm"
                  }
                  option {
                    class: "font-medium",
                    "Red Black Tree"
                  }
                  option {
                    class: "font-medium",
                    "Binomial Heap"
                  }
                }
                // Custom dropdown arrow
                div {
                  class: "absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none
                       text-gray-500",
                  i { class: "fas fa-chevron-down" }
                }
              }
            }

            div {
              class: "w-full space-y-3 border-2 border-gray-200/50 p-6 rounded-xl bg-white/60",
              h3 {
                class: "text-lg font-semibold text-gray-700 flex items-center gap-2",
                "Insert Node"
              }
              div {
                class: "flex flex-col gap-3",
                div {
                  class: "flex items-center gap-3",
                  Input {
                    value: *addNode.read(),
                    placeholder: "Enter a number...",
                    oninput: move |value| {
                      addNode.set(value);
                    }
                  }
                  Button {
                    value: "Insert",
                    color: Some(if *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1 {
                      "bg-gray-500 cursor-not-allowed".to_string()
                    } else {
                      "bg-green-500/80 hover:bg-green-600/80 shadow-sm hover:shadow-md".to_string()
                    }),
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
                        "Binomial Heap" => {
                          // Call Binomial Heap insertion function
                        }
                        _ => {}
                      }

                      TREE_STATES.write().push(RED_BLACK_TREE.read().clone());
                      CONTROLS.write().ind.set(TREE_STATES.read().len() as i32 - 1);
                      addNode.set(0);
                    },
                    disabled: *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1 || *disabled.read()
>>>>>>> 7674bd0 (revert back)
                  }
                }
                p {
                  class: "text-sm text-gray-500 italic",
                  "Enter a number and click to Insert a node"
                }
              }

              h3 {
                class: "text-lg font-semibold text-gray-700 flex items-center gap-2",
                i { class: "fas fa-trash-alt text-red-500" }
                "Delete Node"
              }
              div {
                class: "flex flex-col gap-3",
                div {
                  class: "flex items-center gap-3",
                  Input {
                    value: *deleteNode.read(),
                    placeholder: "Enter a number to delete...",
                    oninput: move |value| {
                      deleteNode.set(value);
                    }
                  }
                  Button {
                    color: Some(if *CONTROLS.read().ind.read() != TREE_STATES.read().len() as i32 - 1 {
                      "bg-gray-500 cursor-not-allowed".to_string()
                    } else {
                      "bg-red-500/80 hover:bg-red-600/80 shadow-sm hover:shadow-md".to_string()
                    }),
                    value: "Delete",
                    onclick: move |_| {
                      let selected_tree = SELECTED_TREE.read().clone();
                      match selected_tree.as_str() {
                        "Red Black Tree" => {
                          // RED_BLACK_TREE.write().delete(&(*deleteNode.read()));
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
                p {
                  class: "text-sm text-gray-500 italic",
                  "Enter a number and click to Delete a node"
                }
              }
            }

            div {
              class: "w-full mt-auto space-y-4",
              div {
                class: "space-y-3 border-2 border-gray-200/50 p-6 rounded-xl bg-white/60 shadow-sm transition-all duration-300 hover:shadow-md",
                h2 {
                  class: "text-center text-lg font-semibold text-gray-700 mb-4",
                  span { class: "flex items-center justify-center gap-2",
                    i { class: "fas fa-tachometer-alt" }
                    "Animation Speed"
                  }
                }
                div {
                  class: "flex flex-col gap-3",
                  div {
                    class: "flex gap-4 items-center",
                    input {
                      class: "flex-1 h-3 rounded-full appearance-none cursor-pointer bg-gradient-to-r from-blue-300 to-blue-500
                          transition-all duration-200 hover:from-blue-400 hover:to-blue-600
                          focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-50",
                      r#type: "range",
                      min: "0",
                      max: "100",
                      value: CONTROLS.read().speed.read().to_string(),
                      oninput: move |e| {
                        CONTROLS.write().speed.set(e.value().parse().unwrap_or(0));
                      },
                    }
                    span {
                      class: "w-16 text-center font-medium p-1 bg-blue-100 rounded-lg text-blue-700",
                      "{CONTROLS.read().speed} ms"
                    }
                  }
                  div {
                    class: "flex justify-between text-sm text-gray-500",
                    span { "Faster" }
                    span { "Slower" }
                  }
                }
              }

                // Navigation Controls
                div {
                    class: "flex gap-3 justify-center",
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
<<<<<<< HEAD
            Button {
              value: "Clear",
              color: Some("bg-gray-500 hover:bg-gray-600 text-white shadow-md".to_string()),
              onclick: move |_| {
                RBTREE.write().clear();
                TREE_STATES.write().clear();
                CONTROLS.write().ind.set(-1);
              },
              disabled: false,
            }
            Button {
              value: "Next →",
              color: Some("bg-blue-500 hover:bg-blue-600 text-white shadow-md".to_string()),
              onclick: move |_| {
                let curr_ind = *CONTROLS.read().ind.read();
                if (curr_ind + 1) < TREE_STATES.read().len() as i32 {
                  let selected_tree = SELECTED_TREE.read().clone();
                  match selected_tree.as_str() {
                    "Red Black Tree" => {
                      *RBTREE.write() = TREE_STATES.read()[(curr_ind + 1) as usize].clone();
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
          }
=======
>>>>>>> 7674bd0 (revert back)
        }
    }
}
