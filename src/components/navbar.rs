use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
      div {
        class: "flex justify-between w-full fixed top-0 px-20 items-center bg-gray-800 h-[3.5rem]",
        div {
            class: "text-white flex justify-center items-center gap-2",
            img {
            class: "h-8 w-8",
            src: "assets/favicon.ico",
            alt: "logo"
            }
            span {
            class: "text-lg font-semibold",
            "Red Black Tree Visualizer"
            }
        }
        div {
          class: "flex gap-2",
          a {
            class: "text-white text-sm hover:bg-gray-300/20 p-1 rounded-lg px-2",
            href: "/",
            "Home"
          }
          a {
            class: "text-white text-sm hover:bg-gray-300/20 p-1 rounded-lg px-2",
            href: "/about",
            "About"
          }
        }
      }
    }
}
