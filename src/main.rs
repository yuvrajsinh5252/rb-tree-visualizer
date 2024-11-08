#![allow(non_snake_case)]

mod algorithm;
mod components;
mod layout;
mod pages;
mod store;

use dioxus::prelude::*;
use layout::Layout;
use pages::home::Home;
use pages::not_found::NotFound;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn main() {
    launch(Layout);
}
