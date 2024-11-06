use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    value: String,
}

pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            r#type: "text",
            placeholder: "{props.value}",
            class: "w-full border-2 p-1 rounded-md",
        }
    }
}
