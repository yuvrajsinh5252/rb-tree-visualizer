use crate::algorithm::tree::Node;
use crate::components::canvas_control::CanvasControls;
use crate::store::RED_BLACK_TREE;
use crate::store::SVG_VIEW_BOX;
use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    let mut red_black_tree = use_signal(|| RED_BLACK_TREE.read().clone());

    use_effect(move || {
        red_black_tree.set(RED_BLACK_TREE.read().clone());
    });

    rsx! {
        div {
            class: "flex flex-col border-2 items-center relative justify-center w-3/4 rounded-lg",
            CanvasControls {}
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
                        class: "transition-all duration-500 ease-in-out transform-gpu origin-center",
                        path {
                            d: "M0,0 L0,3 L3,1.5 z",
                            fill: "black"
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
    let v_gap = 30.0;
    let node_val = node.value;
    let size = node.size;

    let x = node.x;
    let y = node.y;

    let h_gap = 4.0 * (size as f32);

    rsx! {
        g {
            circle {
                cx: x.to_string(),
                cy: y.to_string(),
                r: "8",
                stroke: "black",
                fill: if format!("{:?}", node.color ) == "Red" { "indianred" } else { "gray" },
                class: "transition-all duration-500 ease-in-out transform-gpu origin-center"
            }
            text {
                x: x.to_string(),
                y: (y + 2.0).to_string(),
                text_anchor: "middle",
                fill: "white",
                font_size: "4",
                "{node_val}"
            }

            if let Some(ref left) = &node.left {
                line {
                    x1: (x - 2.0).to_string(),
                    y1: (y + 7.3).to_string(),
                    x2: (x - h_gap).to_string(),
                    y2: (y + v_gap).to_string(),
                    stroke: "black",
                    stroke_width: "0.5",
                    marker_end: "url(#arrowhead)",
                }
                {render_node(left.clone())}
            }

            if let Some(ref right) = &node.right {
                line {
                    x1: (x + 2.0).to_string(),
                    y1: (y + 7.3).to_string(),
                    x2: (x + h_gap).to_string(),
                    y2: (y + v_gap).to_string(),
                    stroke: "black",
                    stroke_width: "0.5",
                    marker_end: "url(#arrowhead)",
                }
                {render_node(right.clone())}
            }
        }
    }
}
