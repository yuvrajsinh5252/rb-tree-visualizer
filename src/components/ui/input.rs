use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    placeholder: String,
    value: i32,
    oninput: EventHandler<i32>,
    error: Option<bool>,
}

pub fn Input(props: InputProps) -> Element {
    let base_classes = "w-full p-2 rounded-lg transition-all duration-200 outline-none";
    let border_classes = match props.error {
        Some(true) => "border-2 border-red-500 focus:border-red-600",
        _ => "border-2 border-gray-300 focus:border-blue-500 hover:border-gray-400",
    };
    let input_classes = format!(
        "{} {} bg-white shadow-sm hover:shadow focus:shadow-md",
        base_classes, border_classes
    );

    rsx! {
        input {
            r#type: "text",
            placeholder: "{props.placeholder}",
<<<<<<< HEAD
            class: input_classes,
=======
            class: "w-full border-2 p-2 rounded-md outline-none transition-all duration-200 focus:border-blue-500 hover:border-gray-400 bg-white/50 backdrop-blur-sm",
>>>>>>> 7674bd0 (revert back)
            value: if props.value != 0 { props.value.to_string() } else { "".to_string() },
            oninput: move |event| {
                if let Ok(value) = event.value().parse::<i32>() {
                    props.oninput.call(value);
                }
            },
        }
    }
}
