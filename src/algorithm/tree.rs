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

    fn is_red(node: Option<&Box<Node<T>>>) -> bool {
        node.map_or(false, |n| n.color == Color::Red)
    }
}

impl<T: Ord + std::fmt::Display + Clone + Into<i32>> RBTree<T> {
    pub fn new() -> Self {
        RBTree { root: None }
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

    pub async fn insert(&mut self, value: T) {
        println!("Inserting value: {}", value);
        if self.root.is_none() {
            let mut new_node = Box::new(Node::new(value));
            new_node.color = Color::Black; // Root must be black
            self.root = Some(new_node);
            self.update_tree_state(true, true, "Created new root node", 1000)
                .await;
            return;
        }

        let root = self.root.take().unwrap();
        self.root = Some(Box::pin(self.insert_recursive(root, value)).await);

        // Ensure root is black
        if let Some(root) = &mut self.root {
            root.color = Color::Black;
        }

        self.update_tree_state(true, true, "Insertion complete", 1000)
            .await;
    }

    async fn insert_recursive(&mut self, mut node: Box<Node<T>>, value: T) -> Box<Node<T>> {
        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(left) = node.left.take() {
                    let new_left = Box::pin(self.insert_recursive(left, value)).await;
                    node.left = Some(new_left);
                } else {
                    self.update_tree_state(true, false, "Creating new left node", 1000)
                        .await;
                    node.left = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(right) = node.right.take() {
                    let new_right = Box::pin(self.insert_recursive(right, value)).await;
                    node.right = Some(new_right);
                } else {
                    self.update_tree_state(true, false, "Creating new right node", 1000)
                        .await;
                    node.right = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Equal => {
                node.value = value;
                return node;
            }
        }

        // Fix Red-Black tree violations
        if Node::is_red(node.right.as_ref()) && !Node::is_red(node.left.as_ref()) {
            node = Box::pin(self.rotate_left(node)).await;
        }
        if Node::is_red(node.left.as_ref())
            && node
                .left
                .as_ref()
                .map_or(false, |n| Node::is_red(n.left.as_ref()))
        {
            node = Box::pin(self.rotate_right(node)).await;
        }
        if Node::is_red(node.left.as_ref()) && Node::is_red(node.right.as_ref()) {
            self.flip_colors(&mut node);
        }

        node
    }

    async fn rotate_left(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        if let Some(mut right) = node.right.take() {
            node.right = right.left.take();
            right.left = Some(node);
            right.color = right.left.as_ref().unwrap().color;
            right.left.as_mut().unwrap().color = Color::Red;
            self.update_tree_state(true, true, "Rotating left", 1000)
                .await;
            right
        } else {
            node
        }
    }

    async fn rotate_right(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        if let Some(mut left) = node.left.take() {
            node.left = left.right.take();
            left.right = Some(node);
            left.color = left.right.as_ref().unwrap().color;
            left.right.as_mut().unwrap().color = Color::Red;
            self.update_tree_state(true, true, "Rotating right", 1000)
                .await;
            left
        } else {
            node
        }
    }

    fn flip_colors(&mut self, node: &mut Box<Node<T>>) {
        node.color = Color::Red;
        if let Some(left) = &mut node.left {
            left.color = Color::Black;
        }
        if let Some(right) = &mut node.right {
            right.color = Color::Black;
        }
    }

    pub async fn delete(&mut self, value: T) {
        if self.root.is_none() {
            self.update_tree_state(true, true, "Tree is empty", 1000)
                .await;
            return;
        }

        let root = self.root.take().unwrap();
        let new_root = Box::pin(self.delete_recursive(root, value)).await;
        self.root = Some(new_root);

        // Ensure root is black
        if let Some(root) = &mut self.root {
            root.color = Color::Black;
        }

        self.update_tree_state(true, true, "Deletion complete", 1000)
            .await;
    }

    async fn delete_recursive(&mut self, mut node: Box<Node<T>>, value: T) -> Box<Node<T>> {
        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(left) = node.left.take() {
                    let new_left = Box::pin(self.delete_recursive(left, value)).await;
                    node.left = Some(new_left);
                    self.balance_after_delete_left(node).await
                } else {
                    node
                }
            }
            Ordering::Greater => {
                if let Some(right) = node.right.take() {
                    let new_right = Box::pin(self.delete_recursive(right, value)).await;
                    node.right = Some(new_right);
                    self.balance_after_delete_right(node).await
                } else {
                    node
                }
            }
            Ordering::Equal => match (node.left.take(), node.right.take()) {
                (None, None) => node,
                (Some(left), None) => {
                    let mut child = left;
                    child.color = Color::Black;
                    child
                }
                (None, Some(right)) => {
                    let mut child = right;
                    child.color = Color::Black;
                    child
                }
                (Some(left), Some(right)) => {
                    let successor = self.find_min(&right);
                    let mut new_node = Box::new(Node::new(successor.clone()));
                    new_node.color = node.color;
                    new_node.left = Some(left);
                    new_node.right = Some(Box::pin(self.delete_recursive(right, successor)).await);
                    self.balance_after_delete_right(new_node).await
                }
            },
        }
    }

    fn find_min(&self, node: &Box<Node<T>>) -> T {
        let mut current = node;
        while let Some(left) = &current.left {
            current = left;
        }
        current.value.clone()
    }

    async fn balance_after_delete_left(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        if Node::is_red(node.right.as_ref()) {
            node = Box::pin(self.rotate_left(node)).await;
        }
        if let Some(right) = &node.right {
            if !Node::is_red(right.left.as_ref()) && !Node::is_red(right.right.as_ref()) {
                self.flip_colors(&mut node);
            } else {
                if Node::is_red(right.left.as_ref()) {
                    if let Some(mut right) = node.right.take() {
                        right = Box::pin(self.rotate_right(right)).await;
                        node.right = Some(right);
                    }
                }
                if let Some(right) = &node.right {
                    if Node::is_red(right.right.as_ref()) {
                        node = Box::pin(self.rotate_left(node)).await;
                        self.flip_colors(&mut node);
                    }
                }
            }
        }
        node
    }

    async fn balance_after_delete_right(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        if Node::is_red(node.left.as_ref()) {
            node = Box::pin(self.rotate_right(node)).await;
        }
        if let Some(left) = &node.left {
            if !Node::is_red(left.left.as_ref()) && !Node::is_red(left.right.as_ref()) {
                self.flip_colors(&mut node);
            } else {
                if Node::is_red(left.right.as_ref()) {
                    if let Some(mut left) = node.left.take() {
                        left = Box::pin(self.rotate_left(left)).await;
                        node.left = Some(left);
                    }
                }
                if let Some(left) = &node.left {
                    if Node::is_red(left.left.as_ref()) {
                        node = Box::pin(self.rotate_right(node)).await;
                        self.flip_colors(&mut node);
                    }
                }
            }
        }
        node
    }

    pub fn update_positions(&mut self) {
        if let Some(root) = &mut self.root {
            Self::update_positions_recursive(root, 100.0, 20.0);
        }
    }

    fn update_positions_recursive(node: &mut Box<Node<T>>, x: f32, y: f32) {
        let v_gap = 35.0;
        let h_gap = 4.5 * (node.size as f32);

        node.x = x;
        node.y = y;

        if let Some(left) = &mut node.left {
            Self::update_positions_recursive(left, x - h_gap, y + v_gap);
        }
        if let Some(right) = &mut node.right {
            Self::update_positions_recursive(right, x + h_gap, y + v_gap);
        }
    }

    pub async fn update_tree_state(&mut self, state: bool, pos: bool, status: &str, delay: i32) {
        let converted_tree: RBTree<i32> = RBTree {
            root: self
                .root
                .as_ref()
                .map(|node| Self::convert_node_to_i32(node)),
        };

        if converted_tree.root.is_some() {
            if state {
                *STATUS.write() = status.to_string();
                if pos {
                    self.update_sizes();
                    self.update_positions();

                    let mut states = TREE_STATES.read().clone();
                    let updated_tree = RBTree {
                        root: self
                            .root
                            .as_ref()
                            .map(|node| Self::convert_node_to_i32(node)),
                    };
                    states.push(updated_tree.clone());
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
