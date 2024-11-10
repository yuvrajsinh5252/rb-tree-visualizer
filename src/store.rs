use crate::algorithm::tree::RBTree;
use dioxus::prelude::*;

pub static RED_BLACK_TREE: GlobalSignal<RBTree<i32, String>> = Signal::global(|| RBTree::new());
pub static ANIMATION_SPEED: GlobalSignal<u32> = Signal::global(|| 1000);
