use crate::store::{CONTROLS, RBTREE, STATUS, TREE_STATES};
use dioxus::prelude::*;
use std::cmp::Ordering;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, window};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone)]
pub struct Node<T: Ord> {
    pub value: T,
    pub color: Color,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub size: usize,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Default)]
pub struct RBTree<T: Ord + Clone> {
    pub root: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            color: Color::Red,
            left: None,
            right: None,
            size: 1,
            x: 0.0,
            y: 0.0,
        }
    }

    fn is_red(node: &Option<Box<Node<T>>>) -> bool {
        node.as_ref().map_or(false, |n| n.color == Color::Red)
    }
}

impl<T: Ord + std::fmt::Display + Clone + Into<i32>> RBTree<T> {
    pub fn new() -> Self {
        RBTree { root: None }
    }

    pub async fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(value)));
            self.update_tree_state(true, true, "Created new root node", 1000)
                .await;
        } else {
            let root = self.root.take().unwrap();
            self.root = Box::pin(self.insert_recursive(Some(root), value)).await;
        }

        if let Some(root) = &mut self.root {
            root.color = Color::Black;
        }

        self.update_tree_state(true, true, "Insertion complete, tree is now balanced", 1000)
            .await;
    }

    pub fn clear_tree(&mut self) {
        self.root = None;
    }

    fn update_sizes(&mut self) {
        fn update_recursive<T: Ord>(node: &mut Option<Box<Node<T>>>) -> usize {
            if let Some(node) = node {
                let left_size = update_recursive(&mut node.left);
                let right_size = update_recursive(&mut node.right);
                node.size = 1 + left_size + right_size;
                node.size
            } else {
                0
            }
        }
        update_recursive(&mut self.root);
    }

    async fn insert_recursive(
        &mut self,
        node: Option<Box<Node<T>>>,
        value: T,
    ) -> Option<Box<Node<T>>> {
        if node.is_none() {
            self.update_tree_state(true, false, "Creating new node", 1000)
                .await;
            return Some(Box::new(Node::new(value)));
        }

        let mut current = node.unwrap();
        match value.cmp(&current.value) {
            Ordering::Less => {
                self.update_tree_state(
                    true,
                    true,
                    &format!("{} is less than {}, moving left", value, current.value),
                    1000,
                )
                .await;
                current.left = Box::pin(self.insert_recursive(current.left.take(), value)).await;
            }
            Ordering::Greater => {
                self.update_tree_state(
                    true,
                    true,
                    &format!("{} is greater than {}, moving right", value, current.value),
                    1000,
                )
                .await;
                current.right = Box::pin(self.insert_recursive(current.right.take(), value)).await;
            }
            Ordering::Equal => {
                self.update_tree_state(
                    true,
                    true,
                    &format!("Value {} already exists", value),
                    1000,
                )
                .await;
                return Some(current);
            }
        }

        if Node::is_red(&current.right) && !Node::is_red(&current.left) {
            self.update_tree_state(true, false, "Performing left rotation for balancing", 0)
                .await;
            current = self.rotate_left(current).await;
        }
        if Node::is_red(&current.left) && Node::is_red(&current.left.as_ref().unwrap().left) {
            self.update_tree_state(true, false, "Performing right rotation for balancing", 0)
                .await;
            current = self.rotate_right(current).await;
        }
        if Node::is_red(&current.left) && Node::is_red(&current.right) {
            self.update_tree_state(true, false, "Flipping colors to maintain black height", 0)
                .await;
            self.flip_colors(&mut current).await;
        }

        Some(current)
    }

    async fn rotate_left(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        let node_val = node.value.clone();
        let mut right = node.right.unwrap();
        node.right = right.left.take();
        right.left = Some(node);
        right.color = right.left.as_ref().unwrap().color;
        right.left.as_mut().unwrap().color = Color::Red;

        self.update_tree_state(
            true,
            true,
            &format!("Rotating left at node {}", node_val),
            1000,
        )
        .await;
        right
    }

    async fn rotate_right(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        let node_val = node.value.clone();
        let mut left = node.left.unwrap();
        node.left = left.right.take();
        left.right = Some(node);
        left.color = left.right.as_ref().unwrap().color;
        left.right.as_mut().unwrap().color = Color::Red;
        self.update_tree_state(
            true,
            true,
            &format!("Rotating right at node {}", node_val),
            1000,
        )
        .await;
        left
    }

    async fn flip_colors(&mut self, node: &mut Box<Node<T>>) {
        let node_val = node.value.clone();
        node.color = Color::Red;
        if let Some(left) = &mut node.left {
            left.color = Color::Black;
        }
        if let Some(right) = &mut node.right {
            right.color = Color::Black;
        }
        self.update_tree_state(
            true,
            false,
            &format!("Flipping colors at node {}", node_val),
            1000,
        )
        .await;
    }

    pub fn update_positions(&mut self) {
        fn update_positions_recursive<T: Ord>(node: &mut Box<Node<T>>, x: f32, y: f32) {
            let v_gap = 35.0;
            let h_gap = 4.5 * (node.size as f32);

            node.x = x;
            node.y = y;

            if let Some(left) = &mut node.left {
                update_positions_recursive(left, x - h_gap, y + v_gap);
            }
            if let Some(right) = &mut node.right {
                update_positions_recursive(right, x + h_gap, y + v_gap);
            }
        }

        if let Some(root) = &mut self.root {
            update_positions_recursive(root, 100.0, 20.0);
        }
    }

    pub async fn update_tree_state(&mut self, state: bool, pos: bool, status: &str, delay: i32)
    where
        T: Clone + Into<i32>,
    {
        let converted_tree: RBTree<i32> = RBTree {
            root: self
                .root
                .as_ref()
                .map(|node| Self::convert_node_to_i32(node)),
        };

        if let Some(_) = converted_tree.root {
            if state {
                *STATUS.write() = status.to_string();
                if pos {
                    self.update_sizes();
                    self.update_positions();

                    let mut states = TREE_STATES.read().clone();
                    states.push(converted_tree.clone());
                    *TREE_STATES.write() = states;
                    CONTROLS.write().ind.set(TREE_STATES.len() as i32 - 1);

                    *RBTREE.write() = converted_tree;

                    if delay > 0 {
                        self.insert_delay(delay).await;
                    }
                }
            }
        }
    }

    fn convert_node_to_i32(node: &Box<Node<T>>) -> Box<Node<i32>> {
        Box::new(Node {
            value: node.value.clone().into(),
            color: node.color,
            left: node.left.as_ref().map(Self::convert_node_to_i32),
            right: node.right.as_ref().map(Self::convert_node_to_i32),
            size: node.size,
            x: node.x,
            y: node.y,
        })
    }

    async fn insert_delay(&self, delay: i32) {
        if delay > 0 {
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay)
                    .unwrap();
            });
            JsFuture::from(promise).await.unwrap();
        }
    }
}
