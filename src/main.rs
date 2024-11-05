#![allow(non_snake_case)]

mod components;
use components::input::Input;
use components::navbar::Navbar;

// use crate::routes::create_routes;
use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            class : "flex flex-col items-center justify-center h-screen",
            Navbar {}
            Input {}
        }
    }
}
