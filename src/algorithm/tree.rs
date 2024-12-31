use crate::store::{RBTREE, STATUS};
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
        self.update_status_and_tree(&format!("Starting insertion of {}", value), 1000)
            .await;

        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(value)));
            self.update_status_and_tree("Created new root node", 1000)
                .await;
        } else {
            let root = self.root.take().unwrap();
            self.root = Box::pin(self.insert_recursive(Some(root), value)).await;
        }

        if let Some(root) = &mut self.root {
            root.color = Color::Black;
            self.update_status_and_tree("Root colored black", 1000)
                .await;
        }

        self.update_status_and_tree("Insertion complete", 1000)
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
        match node {
            None => {
                let new_node = Box::new(Node::new(value));
                self.root = Some(new_node.clone());
                self.root = None;
                Some(new_node)
            }
            Some(mut current) => {
                match value.cmp(&current.value) {
                    Ordering::Less => {
                        self.root = Some(current.clone());
                        self.update_status_and_tree(
                            &format!("Moving left from {}", current.value),
                            1000,
                        )
                        .await;
                        self.root = None;
                        current.left =
                            Box::pin(self.insert_recursive(current.left.take(), value)).await;
                    }
                    Ordering::Greater => {
                        self.root = Some(current.clone());
                        self.update_status_and_tree(
                            &format!("Moving right from {}", current.value),
                            1000,
                        )
                        .await;
                        self.root = None;
                        current.right =
                            Box::pin(self.insert_recursive(current.right.take(), value)).await;
                    }
                    Ordering::Equal => {
                        self.update_status_and_tree(
                            &format!("Value {} already exists", value),
                            1000,
                        )
                        .await;
                        return Some(current);
                    }
                }

                if Node::is_red(&current.right) && !Node::is_red(&current.left) {
                    self.root = Some(current.clone());
                    current = self.rotate_left(current).await;
                    self.update_status_and_tree(
                        &format!("Left rotation at {}", current.value),
                        1000,
                    )
                    .await;
                }
                if Node::is_red(&current.left) && Node::is_red(&current.left.as_ref().unwrap().left)
                {
                    self.root = Some(current.clone());
                    current = self.rotate_right(current).await;
                    self.update_status_and_tree(
                        &format!("Right rotation at {}", current.value),
                        1000,
                    )
                    .await;
                }
                if Node::is_red(&current.left) && Node::is_red(&current.right) {
                    self.root = Some(current.clone());
                    self.flip_colors(&mut current).await;
                }

                Some(current)
            }
        }
    }

    async fn rotate_left(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut right = node
            .right
            .take()
            .expect("Right node must exist for left rotation");
        node.right = right.left.take();
        right.left = Some(node);
        right.color = right.left.as_ref().unwrap().color;
        right.left.as_mut().unwrap().color = Color::Red;

        right
    }

    async fn rotate_right(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut left = node
            .left
            .take()
            .expect("Left node must exist for right rotation");
        node.left = left.right.take();
        left.right = Some(node);
        left.color = left.right.as_ref().unwrap().color;
        left.right.as_mut().unwrap().color = Color::Red;
        left
    }

    async fn flip_colors(&mut self, node: &mut Box<Node<T>>) {
        node.color = Color::Red;
        if let Some(left) = &mut node.left {
            left.color = Color::Black;
        }
        if let Some(right) = &mut node.right {
            right.color = Color::Black;
        }
        self.update_status_and_tree(&format!("Completed color flip"), 1000)
            .await;
    }

    pub fn update_positions(&mut self) {
        fn update_positions_recursive<T: Ord>(node: &mut Box<Node<T>>, x: f32, y: f32) {
            let v_gap = 30.0;
            let h_gap = 4.0 * (node.size as f32);

            node.x = x;
            node.y = y;

            if let Some(left) = &mut node.left {
                if x == 0.0 && y == 0.0 {
                    update_positions_recursive(left, 0.0, 0.0);
                } else {
                    update_positions_recursive(left, x - h_gap, y + v_gap);
                }
            }
            if let Some(right) = &mut node.right {
                if x == 0.0 && y == 0.0 {
                    update_positions_recursive(right, 0.0, 0.0);
                } else {
                    update_positions_recursive(right, x + h_gap, y + v_gap);
                }
            }
        }

        if let Some(root) = &mut self.root {
            update_positions_recursive(root, 100.0, 20.0);
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

    async fn update_status_and_tree(&mut self, status: &str, delay: i32) {
        *STATUS.write() = status.to_string();
        if status.to_string().contains("Moving right from")
            || status.to_string().contains("Moving left from")
        {
            return;
        }

        let converted_tree = RBTree {
            root: self.root.as_ref().map(Self::convert_node_to_i32),
        };
        self.update_sizes();
        self.update_positions();
        *RBTREE.write() = converted_tree;

        self.insert_delay(delay).await;
    }
}
