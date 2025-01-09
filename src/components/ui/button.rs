// components/button.rs
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    value: String,
    onclick: EventHandler<MouseEvent>,
    disabled: Option<bool>,
    color: Option<String>,
}

pub fn Button(props: ButtonProps) -> Element {
    let base_classes = "font-bold py-2 px-4 rounded transition-all duration-200";
    let color_classes = props
        .color
        .clone()
        .unwrap_or("bg-blue-500 hover:bg-blue-700".to_string());
    let disabled_classes = if props.disabled.unwrap_or(false) {
        "opacity-50 cursor-not-allowed"
    } else {
        "hover:shadow-lg active:scale-95"
    };

    rsx! {
        button {

            class: format!("{} {} {}", base_classes, color_classes, disabled_classes),
            r#type: "button",
            onclick: move |event| (props.onclick)(event),
            disabled: props.disabled.unwrap_or(false),
            "{props.value}"
        }
    }
}
