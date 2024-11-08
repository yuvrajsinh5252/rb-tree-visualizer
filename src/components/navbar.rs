use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
      div {
        class: "flex justify-between w-full fixed top-0 px-20 items-center bg-gray-800 h-[3.5rem]",
        div {
          class: "text-white flex justify-center items-center gap-1",
          img {
            class: "h-8",
            src: "/logo.png",
            alt: "logo"
          }
          "Red Black Tree Visualizer"
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
