use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    placeholder: String,
    // oninput: EventHandler<String>,
}

pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            r#type: "text",
            placeholder: "{props.placeholder}",
            class: "w-full border-2 p-1 rounded-md",
            // oninput: move |event| props.oninput.call(String),
        }
    }
}
