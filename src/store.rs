use crate::algorithm::tree::RBTree;
use dioxus::prelude::*;

pub struct Controls {
    pub ind: Signal<i32>,
    pub speed: Signal<i32>,
}

pub static RED_BLACK_TREE: GlobalSignal<RBTree<i32, String>> = Signal::global(|| RBTree::new());
pub static CONTROLS: GlobalSignal<Controls> = Signal::global(|| Controls {
    ind: Signal::new(-1),
    speed: Signal::new(0),
});

pub static TREE_STATES: GlobalSignal<Vec<RBTree<i32, String>>> = Signal::global(|| vec![]);
