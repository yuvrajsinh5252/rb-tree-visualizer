use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
      div {
        class: "flex justify-between items-center bg-gray-800 p-4",
        div {
          class: "text-white",
          "Red Black Tree Visualizer"
        }
        div {
          class: "flex",
          a {
            class: "text-white",
            href: "/",
            "Home"
          }
          a {
            class: "text-white",
            href: "/about",
            "About"
          }
        }
      }
    }
}
