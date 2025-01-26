use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
      div { class: "min-h-screen bg-gradient-to-br from-white via-slate-50 to-white py-20 px-4",
        div { class: "max-w-4xl mx-auto space-y-12",
          // Header section with darker text
          div { class: "text-center space-y-6 animate-fade-in-down",
            h1 { class: "text-6xl font-black text-slate-800", "Red-Black Tree Visualizer" }
            p { class: "text-2xl text-slate-700 font-light max-w-2xl mx-auto",
              "An interactive tool for learning and understanding Red-Black Trees"
            }
          }

          // Features section with improved contrast
          div { class: "bg-white rounded-2xl border p-8 space-y-6 border-slate-200 shadow-lg hover:border-blue-200 transition-all duration-300",
            h2 { class: "text-3xl font-bold text-slate-800", "Features" }
            ul { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
              li { class: "flex items-center space-x-3 text-slate-700 hover:text-slate-900 transition-colors group",
                span { class: "text-blue-600 text-2xl group-hover:translate-x-1 transition-transform",
                  "→"
                }
                span { "Interactive visualization of Red-Black Trees" }
              }
            }
          }

          div { class: "bg-white rounded-2xl p-8 space-y-6 border border-slate-200 shadow-lg",
            h2 { class: "text-3xl font-bold text-slate-800", "Built With" }
            div { class: "flex flex-wrap gap-3",
              div { class: "flex flex-wrap gap-3",
                span { class: "px-6 py-2 bg-blue-50 border border-blue-200 rounded-xl text-slate-700 hover:bg-blue-100 hover:text-slate-900 transition-all duration-300 hover:-translate-y-1",
                  "Rust"
                }
              }
              div { class: "flex flex-wrap gap-3",
                span { class: "px-6 py-2 bg-blue-50 border border-blue-200 rounded-xl text-slate-700 hover:bg-blue-100 hover:text-slate-900 transition-all duration-300 hover:-translate-y-1",
                  "Dioxus"
                }
              }
            }
          }

          div { class: "flex flex-col sm:flex-row justify-center space-y-4 sm:space-y-0 sm:space-x-6 mt-12",
            a {
              class: "group inline-flex items-center justify-center px-8 py-3 bg-slate-800 text-white rounded-xl hover:bg-slate-700 transition-all duration-300 shadow-md hover:shadow-lg font-medium",
              href: "https://github.com/yuvrajsinh5252/RBT-visualizer",
              target: "_blank",
              span { "View on GitHub" }
              span { class: "ml-2 group-hover:translate-x-1 transition-transform",
                "→"
              }
            }
          }
        }
      }
    }
}
