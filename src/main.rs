#![allow(non_snake_case)]

mod components;
mod pages;

use pages::home::Home;

use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            "404 Not Found"
        }
    }
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
