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
                r: "10",
                fill: if node.color == Color::Red { "red" } else { "black" },
            }
            text {
                x: (x - 5.0).to_string(),
                y: (y + 5.0).to_string(),
                fill: "white",
                {node.value.to_string()}
            }
        }

        if node.right.is_some() {
            line {
                x1: (x + 5.0).to_string(),
                y1: (y + 8.0).to_string(),
                x2: (x + offset).to_string(),
                y2: (y + 30.0).to_string(),
                stroke: "black",
                stroke_width: "0.5",
            }
            {render_node(node.right.as_ref().unwrap(), x + offset, y + offset, offset)}
        }
        if node.left.is_some() {
            {render_node(node.left.as_ref().unwrap(), x - offset, y + offset, offset)}
            line {
                x1: (x - 5.0).to_string(),
                y1: (y + 8.0).to_string(),
                x2: (x - offset).to_string(),
                y2: (y + 30.0).to_string(),
                stroke: "black",
                stroke_width: "0.5",
            }
            {render_node(node.left.as_ref().unwrap(), x - offset, y + offset, offset)}
        }
    }
}
