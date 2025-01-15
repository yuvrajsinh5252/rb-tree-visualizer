use crate::algorithm::node::Node;
use crate::components::canvas_control::CanvasControls;
use crate::store::RBTREE;
use crate::store::SVG_VIEW_BOX;
use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    rsx! {
        div {
            class: "flex bg-white flex-col items-center relative justify-center w-full rounded-lg border-2 border-gray-200 shadow-xl",
            CanvasControls {}
            svg {
                class: "overflow-visible p-8",
                width: "100%",
                height: "100%",
                view_box: SVG_VIEW_BOX.read().iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "),
                preserve_aspect_ratio: "xMidYMid meet",

                defs {
                    marker {
                        id: "arrowhead",
                        view_box: "0 0 10 10",
                        ref_x: "2",
                        ref_y: "5",
                        marker_width: "8",
                        marker_height: "8",
                        orient: "auto",
                        class: "transition-all duration-300",
                        path {
                            d: "M0,2 L8,5 L0,8 L2,5 z",
                            fill: "#374151"
                        }
                    }
                }

                {render_node(&*(RBTREE.read().root).borrow(), 100.0, 20.0)}
            }
        }
    }
}

fn render_node(node: &Node<i32>, x: f32, y: f32) -> Element {
    let v_gap = 30.0;
    let h_gap = 4.0 * node.size;

    rsx! {
        g {
             circle {
                cx: x.to_string(),
                cy: y.to_string(),
                r: "8",
                stroke: "black",
                fill: if format!("{:?}", node.color ) == "Red" { "indianred" } else { "gray" },
                class: "transition-all duration-500 ease-in-out transform-gpu origin-center",
            }
            text {
                x: x.to_string(),
                y: (y + 2.0).to_string(),
                text_anchor: "middle",
                fill: "white",
                font_size: "4",
                class: "transition-all duration-500 ease-in-out transform-gpu origin-center",
                "{node.key}"
            }

            if let Some(ref left) = &node.left {
                line {
                    x1: (x - 3.6).to_string(),
                    y1: (y + 7.6).to_string(),
                    x2: (x - h_gap).to_string(),
                    y2: (y + v_gap).to_string(),
                    stroke: "#374151",
                    stroke_width: "0.6",
                    marker_end: "url(#arrowhead)",
                    class: "transition-all duration-300 ease-in-out",
                }
                {render_node(&left.borrow(), x - h_gap, y + v_gap)}
            }

            if let Some(ref right) = &node.right {
                line {
                    x1: (x + 3.6).to_string(),
                    y1: (y + 7.6).to_string(),
                    x2: (x + h_gap).to_string(),
                    y2: (y + v_gap).to_string(),
                    stroke: "#374151",
                    stroke_width: "0.6",
                    marker_end: "url(#arrowhead)",
                    class: "transition-all duration-300 ease-in-out",
                }
                {render_node(&right.borrow(), x + h_gap, y + v_gap)}
            }
        }
    }
}
