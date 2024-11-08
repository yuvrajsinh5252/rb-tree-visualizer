use crate::algorithm::tree::RedBlackTree;
use dioxus::prelude::*;

pub static RED_BLACK_TREE: GlobalSignal<RedBlackTree<i32>> = Signal::global(RedBlackTree::new);
