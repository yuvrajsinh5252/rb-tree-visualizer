use crate::store::RED_BLACK_TREE;
use dioxus::prelude::*;
use web_sys::console;

#[component]
pub fn Canvas() -> Element {
    rsx! {
        div {
            class: "flex flex-col border-2 items-center justify-center w-3/4 rounded-lg overflow-scroll",
            svg {
                width: "100%",
                height: "100%",
                view_box: "-50 0 300 300",
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
                y: (y + 3.0).to_string(),
                text_anchor: "middle",
                fill: "white",
                font_size: "4",
                "{node.key}"
            }

            if let Some(left) = node.left {
                line {
                    x1: x.to_string(),
                    y1: y.to_string(),
                    x2: (x - h_gap).to_string(),
                    y2: (y + v_gap).to_string(),
                    stroke: "black",
                    stroke_width: "0.5",
                }
                {render_node(left, x - h_gap, y + v_gap)}
            }

            if let Some(right) = node.right {
                line {
                    x1: x.to_string(),
                    y1: y.to_string(),
                    x2: (x + h_gap).to_string(),
                    y2: (y + v_gap).to_string(),
                    stroke: "black",
                    stroke_width: "0.5",
                }
                {render_node(right, x + h_gap, y + v_gap)}
            }
        }
    }
}
