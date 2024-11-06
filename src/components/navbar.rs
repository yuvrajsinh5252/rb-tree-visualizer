use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
      div {
        class: "flex justify-between w-full fixed top-0 px-20 items-center bg-gray-800 p-4",
        div {
          class: "text-white",
          "Red Black Tree Visualizer"
        }
        div {
          class: "flex gap-2",
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
