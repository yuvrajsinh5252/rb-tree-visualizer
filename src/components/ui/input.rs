use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    placeholder: String,
    value: i32,
    oninput: EventHandler<i32>,
}

pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            r#type: "text",
            placeholder: "{props.placeholder}",
            class: "w-full border-2 p-1 rounded-md",
            value: if props.value != 0 { props.value.to_string() } else { "".to_string() },
            oninput: move |event| {
                if let Ok(value) = event.value().parse::<i32>() {
                    props.oninput.call(value);
                }
            },
        }
    }
}
