use web_sys::console;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub color: Color,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub size: f32,
}

#[derive(Clone, Default)]
pub struct Tree<T> {
    pub root: Option<Box<Node<T>>>,
}

impl Node<i32> {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            color: Color::Red,
            left: None,
            right: None,
            size: 1.0,
        }
    }
}

impl Tree<i32> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        let mut node = Box::new(Node::new(value));
        if self.root.is_none() {
            self.root = Some(node);
        } else {
            let mut current = self.root.as_mut().unwrap();
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(current);

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();

                if node.left.is_none() {
                    node.left = Some(Box::new(Node::new(value)));
                    break;
                } else {
                    queue.push_back(node.left.as_mut().unwrap());
                }

                if node.right.is_none() {
                    node.right = Some(Box::new(Node::new(value)));
                    break;
                } else {
                    queue.push_back(node.right.as_mut().unwrap());
                }
            }
        }
    }

    pub fn delete(&mut self, value: i32) {
        console::log_1(&"Delete".into());
    }

    pub fn update_sizes(&mut self) {
        fn update_recursive<T>(node: &mut Option<Box<Node<T>>>) -> f32 {
            if let Some(node) = node {
                let left_size = update_recursive(&mut node.left);
                let right_size = update_recursive(&mut node.right);
                node.size = 1.0 + left_size + right_size;
                node.size
            } else {
                0.0
            }
        }
        update_recursive(&mut self.root);
    }

    pub fn clear_tree(&mut self) {
        self.root = None;
    }
}
