use crate::algorithm::tree::RBTree;
use dioxus::prelude::*;

pub struct Controls {
    pub ind: Signal<i32>,
    pub speed: Signal<i32>,
}

pub static SELECTED_TREE: GlobalSignal<String> = Signal::global(|| "not selected".to_string());
pub static RED_BLACK_TREE: GlobalSignal<RBTree<i32, String>> = Signal::global(|| RBTree::new());
pub static CONTROLS: GlobalSignal<Controls> = Signal::global(|| Controls {
    ind: Signal::new(-1),
    speed: Signal::new(0),
});

pub static TREE_STATES: GlobalSignal<Vec<RBTree<i32, String>>> = Signal::global(|| vec![]);
pub static SVG_VIEW_BOX: GlobalSignal<Vec<f32>> =
    Signal::global(|| vec![-50.0, -20.0, 300.0, 300.0]);
pub static STATUS: GlobalSignal<String> = Signal::global(|| "IDLE".to_string());
