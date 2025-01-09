use crate::algorithm::tree::Node;
use crate::algorithm::tree::RBTree;
use crate::components::canvas_control::CanvasControls;
use crate::store::RBTREE;
use crate::store::SVG_VIEW_BOX;
use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    let mut red_black_tree = use_signal(|| RBTree::new());

    use_effect(move || {
        red_black_tree.set(RBTREE.read().clone());
    });

    rsx! {
        div {
            class: "flex flex-col border-2 items-center relative justify-center w-full rounded-lg bg-white shadow-lg p-4",
            CanvasControls {}
            svg {
                class: "overflow-scroll transition-all duration-300 ease-in-out",
                width: "100%",
                height: "100%",
                view_box: SVG_VIEW_BOX.read().iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "),

                defs {
                    marker {
                        id: "arrowhead",
                        view_box: "0 0 10 10",
                        ref_x: "11",
                        ref_y: "1.5",
                        marker_width: "25",
                        marker_height: "15",
                        orient: "auto",
                        class: "transition-all duration-500 ease-in-out",
                        path {
                            d: "M0,0 L0,3 L3,1.5 z",
                            fill: "#374151"
                        }
                    }
                }

                if let Some(ref root) = red_black_tree.read().root {
                    {render_node(root.clone())}
                }
            }
        }
    }
}

fn render_node(node: Box<Node<i32>>) -> Element {
    let v_gap = 35.0;
    let x = node.x;
    let y = node.y;

    let h_gap = 4.5 * (node.size as f32);

    rsx! {
        g {
            circle {
                cx: x.to_string(),
                cy: y.to_string(),
                r: "10",
                stroke: if format!("{:?}", node.color) == "Red" { "#DC2626" } else { "#374151" },
                stroke_width: "1.5",
                fill: if format!("{:?}", node.color) == "Red" { "#FEE2E2" } else { "#F3F4F6" },
                class: "transition-all duration-300 ease-in-out cursor-pointer hover:filter hover:brightness-95",
            }
            text {
                x: x.to_string(),
                y: (y + 2.0).to_string(),
                text_anchor: "middle",
                fill: if format!("{:?}", node.color) == "Red" { "#DC2626" } else { "#374151" },
                font_size: "6",
                font_weight: "bold",
                class: "transition-all duration-300 ease-in-out select-none",
                "{node.value}"
            }

            if let Some(ref left) = &node.left {
                if !(x == 0.0 && y == 0.0) {
                    line {
                        x1: (x - 2.5).to_string(),
                        y1: (y + 8.5).to_string(),
                        x2: (x - h_gap).to_string(),
                        y2: (y + v_gap).to_string(),
                        stroke: "#374151",
                        stroke_width: "1",
                        class: "transition-all duration-300 ease-in-out",
                        marker_end: "url(#arrowhead)",
                    }
                }
                {render_node(left.clone())}
            }

            if let Some(ref right) = &node.right {
                if !(x == 0.0 && y == 0.0) {
                    line {
                        x1: (x + 2.5).to_string(),
                        y1: (y + 8.5).to_string(),
                        x2: (x + h_gap).to_string(),
                        y2: (y + v_gap).to_string(),
                        stroke: "#374151",
                        stroke_width: "1",
                        class: "transition-all duration-300 ease-in-out",
                        marker_end: "url(#arrowhead)",
                    }
                }
                {render_node(right.clone())}
            }
        }
    }
}
