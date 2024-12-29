// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Clone, Copy, PartialEq, Debug)]
// pub enum Color {
//     Red,
//     Black,
// }

// #[derive(Debug)]
// pub struct Node {
//     pub data: i32,
//     pub color: Color,
//     pub left: Option<Rc<RefCell<Node>>>,
//     pub right: Option<Rc<RefCell<Node>>>,
//     pub parent: Option<Rc<RefCell<Node>>>,
// }

// impl Node {
//     fn new(data: i32) -> Node {
//         Node {
//             data,
//             color: Color::Red,
//             left: None,
//             right: None,
//             parent: None,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct RBTree {
//     pub root: Option<Rc<RefCell<Node>>>,
// }

// impl RBTree {
//     pub fn new() -> Self {
//         RBTree { root: None }
//     }

//     fn rotate_left(&mut self, node: Rc<RefCell<Node>>) {
//         let right_child = node.borrow().right.clone().unwrap();

//         node.borrow_mut().right = right_child.borrow().left.clone();
//         if let Some(left) = right_child.borrow().left.clone() {
//             left.borrow_mut().parent = Some(node.clone());
//         }

//         right_child.borrow_mut().parent = node.borrow().parent.clone();

//         match node.borrow().parent.clone() {
//             None => self.root = Some(right_child.clone()),
//             Some(parent) => {
//                 let is_left = parent
//                     .borrow()
//                     .left
//                     .as_ref()
//                     .map_or(false, |left| Rc::ptr_eq(&node, left));
//                 if is_left {
//                     parent.borrow_mut().left = Some(right_child.clone());
//                 } else {
//                     parent.borrow_mut().right = Some(right_child.clone());
//                 }
//             }
//         }

//         right_child.borrow_mut().left = Some(node.clone());
//         node.borrow_mut().parent = Some(right_child);
//     }

//     fn rotate_right(&mut self, node: Rc<RefCell<Node>>) {
//         let left_child = node.borrow().left.clone().unwrap();

//         node.borrow_mut().left = left_child.borrow().right.clone();
//         if let Some(right) = left_child.borrow().right.clone() {
//             right.borrow_mut().parent = Some(node.clone());
//         }

//         left_child.borrow_mut().parent = node.borrow().parent.clone();

//         match node.borrow().parent.clone() {
//             None => self.root = Some(left_child.clone()),
//             Some(parent) => {
//                 let is_left = parent
//                     .borrow()
//                     .left
//                     .as_ref()
//                     .map_or(false, |left| Rc::ptr_eq(&node, left));
//                 if is_left {
//                     parent.borrow_mut().left = Some(left_child.clone());
//                 } else {
//                     parent.borrow_mut().right = Some(left_child.clone());
//                 }
//             }
//         }

//         left_child.borrow_mut().right = Some(node.clone());
//         node.borrow_mut().parent = Some(left_child);
//     }

//     fn fix_insertion(&mut self, mut node: Rc<RefCell<Node>>) {
//         while node.borrow().parent.is_some()
//             && node.borrow().parent.as_ref().unwrap().borrow().color == Color::Red
//         {
//             let parent = node.borrow().parent.clone().unwrap();
//             let grandparent = parent.borrow().parent.clone().unwrap();

//             if Rc::ptr_eq(&parent, grandparent.borrow().left.as_ref().unwrap()) {
//                 let uncle = grandparent.borrow().right.clone();

//                 if let Some(u) = uncle.as_ref() {
//                     if u.borrow().color == Color::Red {
//                         parent.borrow_mut().color = Color::Black;
//                         u.borrow_mut().color = Color::Black;
//                         grandparent.borrow_mut().color = Color::Red;
//                         node = grandparent;
//                         continue;
//                     }
//                 }

//                 if !Rc::ptr_eq(&node, parent.borrow().left.as_ref().unwrap()) {
//                     node = parent.clone();
//                     self.rotate_left(node.clone());
//                 }

//                 let parent = node.borrow().parent.clone().unwrap();
//                 let grandparent = parent.borrow().parent.clone().unwrap();
//                 parent.borrow_mut().color = Color::Black;
//                 grandparent.borrow_mut().color = Color::Red;
//                 self.rotate_right(grandparent);
//             } else {
//                 let uncle = grandparent.borrow().left.clone();

//                 if let Some(u) = uncle.as_ref() {
//                     if u.borrow().color == Color::Red {
//                         parent.borrow_mut().color = Color::Black;
//                         u.borrow_mut().color = Color::Black;
//                         grandparent.borrow_mut().color = Color::Red;
//                         node = grandparent;
//                         continue;
//                     }
//                 }

//                 if !Rc::ptr_eq(&node, parent.borrow().right.as_ref().unwrap()) {
//                     node = parent.clone();
//                     self.rotate_right(node.clone());
//                 }

//                 let parent = node.borrow().parent.clone().unwrap();
//                 let grandparent = parent.borrow().parent.clone().unwrap();
//                 parent.borrow_mut().color = Color::Black;
//                 grandparent.borrow_mut().color = Color::Red;
//                 self.rotate_left(grandparent);
//             }
//         }

//         self.root.as_ref().unwrap().borrow_mut().color = Color::Black;
//     }

//     pub fn insert(&mut self, data: i32) {
//         let new_node = Rc::new(RefCell::new(Node::new(data)));

//         let mut current = self.root.clone();
//         let mut parent = None;

//         while let Some(node) = current.clone() {
//             parent = current.clone();
//             if data < node.borrow().data {
//                 current = node.borrow().left.clone();
//             } else {
//                 current = node.borrow().right.clone();
//             }
//         }

//         new_node.borrow_mut().parent = parent.clone();

//         match parent {
//             None => self.root = Some(new_node.clone()),
//             Some(p) => {
//                 if data < p.borrow().data {
//                     p.borrow_mut().left = Some(new_node.clone());
//                 } else {
//                     p.borrow_mut().right = Some(new_node.clone());
//                 }
//             }
//         }

//         self.fix_insertion(new_node);
//     }
// }

use std::cmp::Ordering;

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
}

#[derive(Debug, Clone, Default)]
pub struct RBTree<T: Ord> {
    pub root: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            color: Color::Red, // New nodes are always red
            left: None,
            right: None,
            size: 1,
        }
    }
}

impl<T: Ord> RBTree<T> {
    pub fn new() -> Self {
        RBTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match self.root.take() {
            None => {
                // First node is always black
                let mut new_node = Node::new(value);
                new_node.color = Color::Black;
                self.root = Some(Box::new(new_node));
            }
            Some(root) => {
                self.root = Some(self.insert_recursive(root, value));
                if let Some(root) = &mut self.root {
                    root.color = Color::Black; // Root must always be black
                }
            }
        }

        self.update_sizes();
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

    fn insert_recursive(&mut self, mut node: Box<Node<T>>, value: T) -> Box<Node<T>> {
        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(left) = node.left.take() {
                    node.left = Some(self.insert_recursive(left, value));
                } else {
                    node.left = Some(Box::new(Node::new(value)));
                }
                self.balance_left(node)
            }
            Ordering::Greater => {
                if let Some(right) = node.right.take() {
                    node.right = Some(self.insert_recursive(right, value));
                } else {
                    node.right = Some(Box::new(Node::new(value)));
                }
                self.balance_right(node)
            }
            Ordering::Equal => node, // Do nothing for duplicate values
        }
    }

    fn balance_left(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        // Handle red-red violation on the left
        if let Some(left) = &node.left {
            if left.color == Color::Red {
                // Left child is red
                if let Some(left_left) = &left.left {
                    if left_left.color == Color::Red {
                        // Left-left case
                        return self.rotate_right(node);
                    }
                }
                if let Some(left_right) = &left.right {
                    if left_right.color == Color::Red {
                        // Left-right case
                        node.left = Some(self.rotate_left(node.left.unwrap()));
                        return self.rotate_right(node);
                    }
                }
            }
        }
        node
    }

    fn balance_right(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        // Handle red-red violation on the right
        if let Some(right) = &node.right {
            if right.color == Color::Red {
                // Right child is red
                if let Some(right_right) = &right.right {
                    if right_right.color == Color::Red {
                        // Right-right case
                        return self.rotate_left(node);
                    }
                }
                if let Some(right_left) = &right.left {
                    if right_left.color == Color::Red {
                        // Right-left case
                        node.right = Some(self.rotate_right(node.right.unwrap()));
                        return self.rotate_left(node);
                    }
                }
            }
        }
        node
    }

    fn rotate_left(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        if let Some(mut right) = node.right.take() {
            node.right = right.left.take();
            right.left = Some(node);
            right.color = right.left.as_ref().unwrap().color;
            right.left.as_mut().unwrap().color = Color::Red;
            right
        } else {
            node
        }
    }

    fn rotate_right(&mut self, mut node: Box<Node<T>>) -> Box<Node<T>> {
        if let Some(mut left) = node.left.take() {
            node.left = left.right.take();
            left.right = Some(node);
            left.color = left.right.as_ref().unwrap().color;
            left.right.as_mut().unwrap().color = Color::Red;
            left
        } else {
            node
        }
    }
}
