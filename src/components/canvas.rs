use crate::components::ui::button::Button;
use crate::store::RED_BLACK_TREE;
use crate::store::SELECTED_TREE;
use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    let mut SVG_VIEW_BOX: Signal<Vec<f32>> = use_signal(|| vec![-50.0, -20.0, 300.0, 300.0]);

    rsx! {
        div {
            class: "flex flex-col border-2 items-center relative justify-center w-3/4 rounded-lg",
            div {
                class: "absolute top-0 right-0 p-1 pr-2",
                h1 {
                    class: "text-sm font-semibold",
                    " Algorithm: {SELECTED_TREE.read()}"
                }
            }
            div {
                class: "absolute top-0 left-0 p-1 pl-2 flex gap-2",
                Button {
                    value: "➕",
                    onclick: move |_| {
                        let mut svg_view_box = SVG_VIEW_BOX.read().clone();
                        let zoom_factor = 0.1;
                        let width_reduction = svg_view_box[2] * zoom_factor;
                        let height_reduction = svg_view_box[3] * zoom_factor;
                        svg_view_box[0] += width_reduction / 2.0;
                        svg_view_box[1] += height_reduction / 2.0;
                        svg_view_box[2] -= width_reduction;
                        svg_view_box[3] -= height_reduction;
                        SVG_VIEW_BOX.set(svg_view_box as Vec<f32>);
                    }
                }
                Button {
                    value: "➖",
                    onclick: move |_| {
                        let mut svg_view_box = SVG_VIEW_BOX.read().clone();
                        let zoom_factor = 0.1;
                        let width_increase = svg_view_box[2] * zoom_factor;
                        let height_increase = svg_view_box[3] * zoom_factor;
                        svg_view_box[0] -= width_increase / 2.0;
                        svg_view_box[1] -= height_increase / 2.0;
                        svg_view_box[2] += width_increase;
                        svg_view_box[3] += height_increase;
                        SVG_VIEW_BOX.set(svg_view_box as Vec<f32>);
                    }
                }
            }

            svg {
                class: "overflow-scroll",
                width: "100%",
                height: "100%",
                view_box: SVG_VIEW_BOX.read().iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "),

                defs {
                    marker {
                        id: "arrowhead",
                        view_box: "0 0 10 10",
                        ref_x: "11",
                        ref_y: "1.5",
                        marker_width: "30",
                        marker_height: "18",
                        orient: "auto",
                        path {
                            d: "M0,0 L0,3 L3,1.5 z",
                            fill: "black"
                        }
                    }
                }

                if let Some(root) = &(*RED_BLACK_TREE.read()).root {
                    {render_node(*root, 100.0, 20.0)}
                }
            }
        }
    }
}

fn render_node(index: usize, x: f32, y: f32) -> Element {
    let v_gap = 30.0;
    let tree = RED_BLACK_TREE.read();
    let node = &tree.nodes[index];

    let h_gap = 4.0 * (node.size as f32);

    rsx! {
        g {
            circle {
                cx: x.to_string(),
                cy: y.to_string(),
                r: "8",
                stroke: "black",
                fill: if format!("{:?}", node.color ) == "RED" { "indianred" } else { "gray" },
            }
            text {
                x: x.to_string(),
                y: (y + 2.0).to_string(),
                text_anchor: "middle",
                fill: "white",
                font_size: "4",
                "{node.key}"
            }

            if let Some(left) = node.left {
                line {
                    x1: (x - 2.0).to_string(),
                    y1: (y + 7.3).to_string(),
                    x2: (x - h_gap).to_string(),
                    y2: (y + v_gap).to_string(),
                    stroke: "black",
                    stroke_width: "0.5",
                    marker_end: "url(#arrowhead)",
                }
                {render_node(left, x - h_gap, y + v_gap)}
            }

            if let Some(right) = node.right {
                line {
                    x1: (x + 2.0).to_string(),
                    y1: (y + 7.3).to_string(),
                    x2: (x + h_gap).to_string(),
                    y2: (y + v_gap).to_string(),
                    stroke: "black",
                    stroke_width: "0.5",
                    marker_end: "url(#arrowhead)",
                }
                {render_node(right, x + h_gap, y + v_gap)}
            }
        }
    }
}
