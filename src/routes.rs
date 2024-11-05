use super::pages;
use dioxus::prelude::*;
use pages::{add_task, create_account, edit_task, home, login, one_task};

#[derive(PartialEq)]
pub enum AppRoute {
    Home,
    CreateAccount,
    Login,
    OneTask { id: usize },
    EditTask { id: usize },
    AddTask,
}

pub fn create_routes(cx: Scope) -> Element {
    cx.render(rsx! {
        Route { to: "/", home::home {}}
        Route { to: "/create-account", create_account::create_account{}}
        Route { to: "/login", login::login{}}
        Route { to: "/tasks/add", add_task::add_task{}}
        Route { to: "/tasks/:id", one_task::one_task{} }
        Route { to: "/tasks/:id/edit", edit_task::edit_task{}}
    })
}
