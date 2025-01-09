use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
      div {
        class: "flex justify-between w-full fixed top-0 px-8 md:px-20 items-center bg-gray-800 h-[3.5rem] shadow-lg z-50",
        div {
            class: "text-white flex justify-center items-center gap-3 hover:opacity-80 transition-opacity cursor-pointer",
            img {
                class: "h-9 w-9",
                src: "assets/favicon.ico",
                alt: "logo"
            }
            span {
                class: "text-xl font-bold tracking-tight hidden md:block",
                "Red Black Tree Visualizer"
            }
        }
        div {
            class: "flex gap-4",
            a {
                class: "text-gray-200 text-sm font-medium hover:bg-gray-700 hover:text-white transition-colors p-2 rounded-md px-4 relative after:absolute after:bottom-0 after:left-0 after:h-0.5 after:bg-red-500 after:w-full after:scale-x-0 hover:after:scale-x-100 after:transition-transform",
                href: "/",
                "Home"
            }
            a {
                class: "text-gray-200 text-sm font-medium hover:bg-gray-700 hover:text-white transition-colors p-2 rounded-md px-4 relative after:absolute after:bottom-0 after:left-0 after:h-0.5 after:bg-red-500 after:w-full after:scale-x-0 hover:after:scale-x-100 after:transition-transform",
                href: "/about",
                "About"
            }
        }
      }
    }
}
