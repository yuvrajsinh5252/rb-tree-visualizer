use crate::store::STATUS;
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
pub struct RBTree<T: Ord> {
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

impl<T: Ord + std::fmt::Display> RBTree<T> {
    pub fn new() -> Self {
        RBTree { root: None }
    }

    pub async fn insert(&mut self, value: T) {
        self.update_status("Inserting node...", 1000).await;

        self.root = Self::insert_recursive(self.root.take(), value);
        if let Some(root) = &mut self.root {
            root.color = Color::Black;
        }
        self.update_sizes();
        self.update_positions();

        self.update_status("IDLE", 0).await;
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

    fn insert_recursive(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        if node.is_none() {
            return Some(Box::new(Node::new(value)));
        }

        let mut current = node.unwrap();
        match value.cmp(&current.value) {
            Ordering::Less => {
                current.left = Self::insert_recursive(current.left.take(), value);
            }
            Ordering::Greater => {
                current.right = Self::insert_recursive(current.right.take(), value);
            }
            Ordering::Equal => return Some(current),
        }

        if Node::is_red(&current.right) && !Node::is_red(&current.left) {
            current = Self::rotate_left(current);
        }
        if Node::is_red(&current.left) && Node::is_red(&current.left.as_ref().unwrap().left) {
            current = Self::rotate_right(current);
        }
        if Node::is_red(&current.left) && Node::is_red(&current.right) {
            Self::flip_colors(&mut current);
        }

        Some(current)
    }

    fn rotate_left(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut right = node.right.unwrap();
        node.right = right.left.take();
        right.left = Some(node);
        right.color = right.left.as_ref().unwrap().color;
        right.left.as_mut().unwrap().color = Color::Red;
        right
    }

    fn rotate_right(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut left = node.left.unwrap();
        node.left = left.right.take();
        left.right = Some(node);
        left.color = left.right.as_ref().unwrap().color;
        left.right.as_mut().unwrap().color = Color::Red;
        left
    }

    fn flip_colors(node: &mut Box<Node<T>>) {
        node.color = Color::Red;
        if let Some(left) = &mut node.left {
            left.color = Color::Black;
        }
        if let Some(right) = &mut node.right {
            right.color = Color::Black;
        }
    }

    pub fn update_positions(&mut self) {
        fn update_positions_recursive<T: Ord>(node: &mut Box<Node<T>>, x: f32, y: f32) {
            let v_gap = 30.0;
            let h_gap = 4.0 * (node.size as f32);

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

    async fn update_status(&self, status: &str, delay: i32) {
        *STATUS.write() = status.to_string();

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
