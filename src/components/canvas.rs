use crate::{
    algorithm::tree::{Color, Node},
    store::RED_BLACK_TREE,
};
use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    rsx! {
        div {
            class: "flex flex-col border-2 items-center justify-center w-3/4 rounded-lg",
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 15 100 100",
                if let Some(root) = &(*RED_BLACK_TREE.read()).root {
                    {render_node(root, 50.0, 30.0, 30.0)}
                }
            }
        }
    }
}

fn render_node(node: &Box<Node<i32>>, x: f32, y: f32, offset: f32) -> Element {
    rsx! {
        g {
            circle {
                cx: x.to_string(),
                cy: y.to_string(),
                r: "5",
                fill: if node.color == Color::Red { "red" } else { "black" },
            }
            text {
                x: x.to_string(),
                y: (y + 2.0).to_string(),
                text_anchor: "middle",
                fill: "white",
                font_size: "5",
                {node.value.to_string()}
            }
        }

        if let Some(right) = &node.right {
            line {
                x1: (x + 3.2).to_string(),
                y1: (y + 3.2).to_string(),
                x2: (x + offset).to_string(),
                y2: (y + offset).to_string(),
                stroke: "black",
                stroke_width: "0.5",
            }
            {render_node(right, x + offset, y + offset, offset)}
        }
        if let Some(left) = &node.left {
            line {
                x1: (x - 3.2).to_string(),
                y1: (y + 3.2).to_string(),
                x2: (x - offset).to_string(),
                y2: (y + offset).to_string(),
                stroke: "black",
                stroke_width: "0.5",
            }
            {render_node(left, x - offset, y + offset, offset)}
        }
    }
}
