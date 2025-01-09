use crate::algorithm::tree::Tree;
use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct Controls {
    pub ind: Signal<i32>,
    pub speed: Signal<i32>,
}

pub static SELECTED_TREE: GlobalSignal<String> = Signal::global(|| "not selected".to_string());
pub static RED_BLACK_TREE: GlobalSignal<Tree<i32>> = Signal::global(|| Tree::new()); // Global tree
pub static RBTREE: GlobalSignal<Tree<i32>> = Signal::global(|| Tree::new()); // Tree for rendering and showing intermediate steps
pub static CONTROLS: GlobalSignal<Controls> = Signal::global(|| Controls {
    ind: Signal::new(-1),
    speed: Signal::new(0),
});

// To implement the prev and next button functionality
pub static TREE_STATES: GlobalSignal<Vec<Tree<i32>>> = Signal::global(|| vec![]);
pub static SVG_VIEW_BOX: GlobalSignal<Vec<f32>> =
    Signal::global(|| vec![-50.0, -20.0, 300.0, 300.0]);
pub static STATUS: GlobalSignal<String> = Signal::global(|| "IDLE".to_string());
