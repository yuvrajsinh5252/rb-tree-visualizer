use dioxus::prelude::*;

#[derive(Clone)]
struct TreeNode {
    value: i32,
    color: String, // "black" or "red"
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

#[component]
pub fn Canvas() -> Element {
    // Example tree
    let tree = Some(Box::new(TreeNode {
        value: 1,
        color: "black".into(),
        left: Some(Box::new(TreeNode {
            value: 2,
            color: "black".into(),
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            color: "black".into(),
            left: None,
            right: Some(Box::new(TreeNode {
                value: 4,
                color: "red".into(),
                left: None,
                right: None,
            })),
        })),
    }));

    rsx! {
        div {
            class: "flex flex-col border-2 items-center justify-center w-3/4 rounded-lg",
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 15 100 100",
                {render_node(&tree, 50.0, 30.0, 20.0)}
            }
        }
    }
}

fn render_node(node: &Option<Box<TreeNode>>, x: f32, y: f32, offset: f32) -> Element {
    rsx! {
        if let Some(node) = node {
            // Render current node
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "5",
                fill: "{node.color}",
            }
            text {
                x: "{x}",
                y: "{y}",
                dy: ".3em",
                text_anchor: "middle",
                fill: "white",
                font_size: "0.5em",
                "{node.value}"
            }
            // Render lines and recursive nodes
            if let Some(left) = &node.left {
                line {
                    x1: "{x}",
                    y1: "{y + 5.0}",
                    x2: "{x - offset}",
                    y2: "{y + 30.0}",
                    stroke: "black",
                    stroke_width: "0.5",
                }
                { render_node(&Some(left.clone()), x - offset, y + 30.0, offset / 1.5) }
            }
            if let Some(right) = &node.right {
                line {
                    x1: "{x}",
                    y1: "{y + 5.0}",
                    x2: "{x + offset}",
                    y2: "{y + 30.0}",
                    stroke: "black",
                    stroke_width: "0.5",
                }
                { render_node(&Some(right.clone()), x + offset, y + 30.0, offset / 1.5) }
            }
        }
    }
}
