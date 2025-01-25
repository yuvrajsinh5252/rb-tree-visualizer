use crate::store::{SELECTED_TREE, STATUS, SVG_VIEW_BOX};
use dioxus::prelude::*;

#[component]
pub fn CanvasControls() -> Element {
    let mut state = use_signal(|| String::from("initial state"));

    use_effect(move || {
        let status = STATUS.read().to_string();
        state.set(status);
    });

    rsx! {
        div {
<<<<<<< HEAD
            class: "absolute top-4 right-4 bg-white/90 rounded-lg border-2 p-2",
            h1 {
                class: "text-sm font-semibold text-gray-700 flex items-center",
                span { class: "mr-2 text-blue-500", "🌳" }
                "Algorithm: "
                span { class: "ml-1 text-blue-600", "{SELECTED_TREE.read()}" }
=======
            class: "absolute top-4 right-4 bg-white/80 backdrop-blur-sm rounded-lg shadow-md px-4 py-2",
            h1 {
                class: "text-sm font-semibold text-gray-700",
                "Algorithm: "
                span { class: "text-blue-600", "{SELECTED_TREE.read()}" }
>>>>>>> 7674bd0 (revert back)
            }
        }

        div {
<<<<<<< HEAD
            class: "absolute top-4 left-4 bg-white/90 rounded-lg border-2 p-2",
            h1 {
                class: "text-sm font-semibold text-gray-700 flex items-center",
                span { class: "mr-2 text-green-500", "📊" }
                "Status: "
                span {
                    class: "ml-1 text-green-600",
=======
            class: "absolute top-4 left-4 bg-white/80 backdrop-blur-sm rounded-lg shadow-md px-4 py-2",
            h1 {
                class: "text-sm font-semibold flex items-center gap-2",
                "Status: "
                span {
                    class: "px-2 py-1 bg-gray-100 rounded-md text-gray-700",
>>>>>>> 7674bd0 (revert back)
                    "{state}"
                }
            }
        }
<<<<<<< HEAD

        div {
            class: "absolute bottom-4 right-4 p-2 flex gap-2",
            button {
                class: "w-8 h-8 bg-blue-500 hover:bg-blue-600 text-white rounded",
=======
        div {
            class: "absolute bottom-4 right-4 flex gap-2",
            Button {
                value: "+",
                color: "bg-blue-600 p-2",
>>>>>>> 7674bd0 (revert back)
                onclick: move |_| {
                    let mut svg_view_box = SVG_VIEW_BOX.read().clone();
                    let zoom_factor = 0.1;
                    let width_reduction = svg_view_box[2] * zoom_factor;
                    let height_reduction = svg_view_box[3] * zoom_factor;
                    svg_view_box[0] += width_reduction / 2.0;
                    svg_view_box[1] += height_reduction / 2.0;
                    svg_view_box[2] -= width_reduction;
                    svg_view_box[3] -= height_reduction;
                    *SVG_VIEW_BOX.write() = svg_view_box as Vec<f32>;
                },
                "+"
            }
<<<<<<< HEAD
            button {
                class: "w-8 h-8 bg-blue-500 hover:bg-blue-600 text-white rounded",
=======
            Button {
                value: "-",
                color: "bg-blue-600 p-2",
>>>>>>> 7674bd0 (revert back)
                onclick: move |_| {
                    let mut svg_view_box = SVG_VIEW_BOX.read().clone();
                    let zoom_factor = 0.1;
                    let width_increase = svg_view_box[2] * zoom_factor;
                    let height_increase = svg_view_box[3] * zoom_factor;
                    svg_view_box[0] -= width_increase / 2.0;
                    svg_view_box[1] -= height_increase / 2.0;
                    svg_view_box[2] += width_increase;
                    svg_view_box[3] += height_increase;
                    *SVG_VIEW_BOX.write() = svg_view_box as Vec<f32>;
                },
                "-"
            }
        }
    }
}
