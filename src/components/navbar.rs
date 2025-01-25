use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
      nav { class: "flex justify-between w-full fixed top-0 px-20 items-center bg-gray-800 h-[3.5rem] shadow-lg z-50 transition-all duration-200",
        div { class: "text-white flex justify-center items-center gap-3 hover:opacity-80 transition-opacity cursor-pointer",
          img {
            class: "h-8 w-8 transform hover:scale-105 transition-transform",
            src: "assets/rbtree.png",
            alt: "logo",
          }
          // i { class: "fas fa-tree text-blue-500 text-2xl" }
          span { class: "text-lg font-bold tracking-wide", "Red Black Tree Visualizer" }
        }
        div { class: "flex gap-4",
          a {
            class: "text-white text-sm hover:bg-gray-700 hover:text-gray-200 p-2 rounded-md px-4 transition-all duration-200 font-medium",
            href: "/",
            "Home"
          }
          a {
            class: "text-white text-sm hover:bg-gray-700 hover:text-gray-200 p-2 rounded-md px-4 transition-all duration-200 font-medium",
            href: "/about",
            "About"
          }
        }
      }
    }
}
