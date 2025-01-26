use crate::algorithm::tree::{Color, Pointer, RBTree};
use crate::components::canvas_control::CanvasControls;
use crate::store::{RBTREE, SVG_VIEW_BOX};
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

                {
                    let tree = red_black_tree.read();
                    if !tree.root.is_null() {
                        render_tree(&tree, tree.root, 100.0, 20.0)
                    } else {
                        rsx! {
                            g {}
                        }
                    }
                }
            }
        }
    }
}

fn render_tree(tree: &RBTree, pointer: Pointer, x: f32, y: f32) -> Element {
    if pointer.is_null() {
        return rsx! {
            g {}
        };
    }

    let node = &tree[pointer];
    let v_gap = 35.0;
    let h_gap = 4.5 * (node.size as f32);

    rsx! {
        g {
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "10",
                stroke: if node.color == Color::Red { "#991b1b" } else { "#1f2937" },
                stroke_width: "1.5",
                fill: if node.color == Color::Red { "#ef4444" } else { "#475569" },
                class: "transition-all duration-300 ease-in-out cursor-pointer hover:filter hover:brightness-110 hover:shadow-lg",
            }
            text {
                x: "{x}",
                y: "{y + 2.5}",
                text_anchor: "middle",
                fill: "white",
                font_size: "6",
                font_weight: "bold",
                class: "transition-all duration-300 ease-in-out select-none pointer-events-none",
                "{node.value}"
            }

            {
                if !node.left.is_null() {
                    rsx! {
                        g {
                            line {
                                x1: "{x - 2.5}",
                                y1: "{y + 8.5}",
                                x2: "{x - h_gap}",
                                y2: "{y + v_gap}",
                                stroke: "#475569",
                                stroke_width: "0.8",
                                class: "transition-all duration-300 ease-in-out",
                                marker_end: "url(#arrowhead)",
                            }
                            {render_tree(tree, node.left, x - h_gap, y + v_gap)}
                        }
                    }
                } else {
                    rsx! {
                        g {}
                    }
                }
            }

            {
                if !node.right.is_null() {
                    rsx! {
                        g {
                            line {
                                x1: "{x + 2.5}",
                                y1: "{y + 8.5}",
                                x2: "{x + h_gap}",
                                y2: "{y + v_gap}",
                                stroke: "#475569",
                                stroke_width: "0.8",
                                class: "transition-all duration-300 ease-in-out",
                                marker_end: "url(#arrowhead)",
                            }
                            {render_tree(tree, node.right, x + h_gap, y + v_gap)}
                        }
                    }
                } else {
                    rsx! {
                        g {}
                    }
                }
            }
        }
    }
}
