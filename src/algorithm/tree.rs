use dioxus::prelude::*;
use dioxus::signals::Writable;
use slab::Slab;
use std::ops::{Index, IndexMut};

use crate::store::{CONTROLS, TREE_STATES};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Pointer(usize);

impl Pointer {
    #[inline]
    pub fn null() -> Pointer {
        Pointer(!0)
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

impl Index<Pointer> for RBTree {
    type Output = Node;

    fn index(&self, index: Pointer) -> &Node {
        &self.slab[index.0]
    }
}

impl IndexMut<Pointer> for RBTree {
    fn index_mut(&mut self, index: Pointer) -> &mut Node {
        &mut self.slab[index.0]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub value: u32,
    pub right: Pointer,
    pub left: Pointer,
    pub parent: Pointer,
    pub color: Color,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct RBTree {
    pub slab: Slab<Node>,
    pub root: Pointer,
}

impl RBTree {
    pub fn new() -> Self {
        RBTree {
            slab: Slab::new(),
            root: Pointer::null(),
        }
    }

    pub fn clear_tree(&mut self) {
        self.slab.clear();
        self.root = Pointer::null();
    }

    pub fn insert(&mut self, val: u32) {
        if self.root.is_null() {
            self.root = Pointer(self.slab.insert(Node {
                value: val,
                right: Pointer::null(),
                left: Pointer::null(),
                parent: Pointer::null(),
                color: Color::Black,
                size: 1,
            }));
        } else {
            let new_node = self.insert_node(val, self.root);
            if !new_node.is_null() {
                self.insert_fixup(new_node);
            }
        }

        self.update_sizes();
        TREE_STATES.write().push(self.clone());
        CONTROLS
            .write()
            .ind
            .set(TREE_STATES.read().len() as i32 - 1);
    }

    fn update_sizes(&mut self) {
        fn update_recursive(tree: &mut RBTree, node: Pointer) -> u32 {
            if node.is_null() {
                return 0;
            }

            let left_size = update_recursive(tree, tree[node].left);
            let right_size = update_recursive(tree, tree[node].right);
            tree[node].size = 1 + left_size + right_size;
            tree[node].size
        }
        update_recursive(self, self.root);
    }

    fn insert_fixup(&mut self, node: Pointer) {
        let parent = self[node].parent;
        if self[node].parent.is_null() {
            return self.insert_case1(node);
        }

        if self[parent].color == Color::Black {
            return self.insert_case2(node);
        }

        let uncle = self.uncle(node);

        if uncle.is_null() {
            return self.insert_case4(node);
        }
        if self[uncle].color == Color::Black {
            return self.insert_case4(node);
        }

        return self.insert_case3(node);
    }

    fn insert_case1(&mut self, node: Pointer) {
        self[node].color = Color::Black;
    }

    fn insert_case2(&mut self, _node: Pointer) {
        return;
    }

    fn insert_case3(&mut self, node: Pointer) {
        let parent = self[node].parent;
        let uncle = self.uncle(node);
        let grandparent = self[parent].parent;

        self[parent].color = Color::Black;
        self[uncle].color = Color::Black;
        self[grandparent].color = Color::Red;

        self.insert_fixup(grandparent);
    }

    fn insert_case4(&mut self, node: Pointer) {
        let parent = self[node].parent;
        let grandparent = self[parent].parent;

        let parent_left = self[parent].left;
        let parent_right = self[parent].right;

        let grandparent_left = self[grandparent].left;
        let grandparent_right = self[grandparent].right;

        let mut n = node;

        if !parent_right.is_null()
            && !grandparent_left.is_null()
            && (self[n].value == self[parent_right].value)
            && (self[parent].value == self[grandparent_left].value)
        {
            self.rotate_left(parent);
            n = self[n].left;
        } else if !parent_left.is_null()
            && !grandparent_right.is_null()
            && (self[n].value == self[parent_left].value)
            && (self[parent].value == self[grandparent_right].value)
        {
            self.rotate_right(parent);
            n = self[n].right;
        }

        let parent = self[n].parent;
        let grandparent = self[parent].parent;

        let parent_left = self[parent].left;

        if !parent_left.is_null() && self[n].value == self[parent_left].value {
            self.rotate_right(grandparent);
        } else {
            self.rotate_left(grandparent);
        }

        self[parent].color = Color::Black;
        self[grandparent].color = Color::Red;
    }

    fn uncle(&self, node: Pointer) -> Pointer {
        let parent = self[node].parent;
        if parent.is_null() {
            return Pointer::null();
        }

        let grandparent = self[parent].parent;

        if grandparent.is_null() {
            return Pointer::null();
        }

        let grandparent_left = self[grandparent].left;
        let grandparent_right = self[grandparent].right;

        if grandparent_left.is_null() || grandparent_right.is_null() {
            return Pointer::null();
        }

        if self[parent].value == self[grandparent_left].value {
            return grandparent_right;
        }

        return grandparent_left;
    }

    fn insert_node(&mut self, val: u32, node: Pointer) -> Pointer {
        let node_value = self[node].value;
        let left = self[node].left;
        let right = self[node].right;

        if val == node_value {
            return Pointer::null();
        } else if val > node_value {
            if right.is_null() {
                self[node].right = Pointer(self.slab.insert(Node {
                    value: val,
                    right: Pointer::null(),
                    left: Pointer::null(),
                    parent: node,
                    color: Color::Red,
                    size: 1,
                }));
                return self[node].right;
            } else {
                return self.insert_node(val, right);
            }
        } else if left.is_null() {
            self[node].left = Pointer(self.slab.insert(Node {
                value: val,
                right: Pointer::null(),
                left: Pointer::null(),
                parent: node,
                color: Color::Red,
                size: 1,
            }));
            return self[node].left;
        } else {
            return self.insert_node(val, left);
        }
    }

    fn rotate_left(&mut self, current: Pointer) {
        let right = self[current].right;

        if right.is_null() {
            return;
        }

        let right_left = self[right].left;
        let parent = self[current].parent;

        self[current].right = right_left;

        if !right_left.is_null() {
            self[right_left].parent = current;
        }

        self[current].parent = right;
        self[right].left = current;

        self[right].parent = parent;

        if parent.is_null() {
            self.root = right;
        } else {
            let parent_right = self[parent].right;
            if parent_right.is_null() {
                self[parent].left = right;
            } else if self[parent_right].value == self[current].value {
                self[parent].right = right;
            } else {
                self[parent].left = right;
            }
        }
    }

    fn rotate_right(&mut self, current: Pointer) {
        let left = self[current].left;

        if left.is_null() {
            return;
        }

        let left_right = self[left].right;
        let parent = self[current].parent;

        self[current].left = left_right;

        if !left_right.is_null() {
            self[left_right].parent = current;
        }

        self[current].parent = left;
        self[left].right = current;

        self[left].parent = parent;

        if parent.is_null() {
            self.root = left;
        } else {
            let parent_left = self[parent].left;
            if parent_left.is_null() {
                self[parent].right = left;
            } else if self[parent_left].value == self[current].value {
                self[parent].left = left;
            } else {
                self[parent].right = left;
            }
        }
    }
}
