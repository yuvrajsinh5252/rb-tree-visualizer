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
        div { class: "flex relative flex-col items-center justify-center w-full max-sm:min-h-[75%] rounded-xl bg-gradient-to-br from-slate-50 to-slate-100 shadow-lg p-4 border border-slate-200",
            CanvasControls {}
            svg {
                class: "overflow-scroll p-4",
                width: "100%",
                height: "100%",
                view_box: SVG_VIEW_BOX.read().iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "),

                defs {
                    marker {
                        id: "arrowhead",
                        view_box: "0 0 10 10",
                        ref_x: "13.2",
                        ref_y: "1.5",
                        marker_width: "12",
                        marker_height: "16",
                        orient: "auto",
                        class: "transition-all duration-300 ease-in-out",
                        path { d: "M0,0 L0,3 L3,1.5 z", fill: "#475569" }
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
                stroke: if format!("{:?}", node.color) == "Red" { "#991b1b" } else { "#1f2937" },
                stroke_width: "1.5",
                fill: if format!("{:?}", node.color) == "Red" { "#ef4444" } else { "#475569" },
                class: "transition-all duration-300 ease-in-out cursor-pointer hover:filter hover:brightness-110 hover:shadow-lg",
            }
            text {
                x: x.to_string(),
                y: (y + 2.5).to_string(),
                text_anchor: "middle",
                fill: "white",
                font_size: "6",
                font_weight: "bold",
                class: "transition-all duration-300 ease-in-out select-none pointer-events-none",
                "{node.value}"
            }

            if let Some(ref left) = &node.left {
                if !(x == 0.0 && y == 0.0) {
                    line {
                        x1: (x - 2.5).to_string(),
                        y1: (y + 8.5).to_string(),
                        x2: (x - h_gap).to_string(),
                        y2: (y + v_gap).to_string(),
                        stroke: "#475569",
                        stroke_width: "0.8",
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
                        stroke: "#475569",
                        stroke_width: "0.8",
                        class: "transition-all duration-300 ease-in-out",
                        marker_end: "url(#arrowhead)",
                    }
                }
                {render_node(right.clone())}
            }
        }
    }
}
