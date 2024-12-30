use crate::components::ui::button::Button;
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
          class: "absolute top-0 right-0 p-1 pr-2",
          h1 {
              class: "text-sm font-semibold",
              " Algorithm: {SELECTED_TREE.read()}"
            }
        }
        div {
            class: "absolute top-0 left-0 p-1 pl-2 flex gap-2",
            h1 {
                class: "p-1 border-1 text-sm font-semibold",
                "Status: {state}"
            }
        }
        div {
            class: "absolute bottom-0 right-0 p-1 pl-2 flex gap-2",
            Button {
                value: "➕",
                color: "bg-gray-200",
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
                }
            }
            Button {
                value: "➖",
                color: "bg-gray-200",
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
                }
            }
        }
    }
}
