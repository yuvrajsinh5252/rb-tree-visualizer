// components/button.rs
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    value: String,
}
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
      button {
        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
        r#type: "button",
        "{props.value}"
      }
    }
}
