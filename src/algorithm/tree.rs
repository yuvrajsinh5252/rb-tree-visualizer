use crate::algorithm::node::{Node, NodeColor};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Default, Clone, Debug)]
pub struct Tree<T> {
    pub root: Rc<RefCell<Node<T>>>,
    pub sentinel: Rc<RefCell<Node<T>>>,
    pub length: usize,
}

// TODO cleanup traits. For instance, debug might be to strict.
impl<T: PartialOrd + Clone + PartialEq + Debug + Default> Tree<T> {
    pub fn new() -> Tree<T> {
        let sentinel = Rc::new(RefCell::new(Node::new_sentinel()));
        Self {
            root: sentinel.clone(),
            sentinel,
            length: 0,
        }
    }

    pub fn insert(&mut self, key: T) {
        let mut z = Node::new(key);
        let mut x = self.root.clone();
        let mut y = self.sentinel.clone();

        while !x.borrow().is_nil() {
            y = x.clone();
            if z.key < x.borrow().key {
                let x_tmp = x.borrow().left().clone();
                x = x_tmp
            } else {
                let x_tmp = x.borrow().right().clone();
                x = x_tmp;
            }
        }
        z.set_parent(y.clone());
        // Z is now Reference counted for
        let z = Rc::new(RefCell::new(z));

        if y.borrow().is_nil() {
            self.root = z.clone();
        } else if z.borrow().key < y.borrow().key {
            y.borrow_mut().set_left_child(z.clone());
        } else {
            y.borrow_mut().set_right_child(z.clone());
        }

        z.borrow_mut().set_left_child(self.sentinel.clone());
        z.borrow_mut().set_right_child(self.sentinel.clone());
        z.borrow_mut().color = NodeColor::Red;
        self.insert_fix_up(z);
        self.length += 1;
    }

    fn insert_fix_up(&mut self, mut z: Rc<RefCell<Node<T>>>) {
        while z.borrow().parent().borrow().color == NodeColor::Red {
            if z.borrow().parent() == z.borrow().parent().borrow().parent().borrow().left() {
                let y = z
                    .borrow()
                    .parent()
                    .borrow()
                    .parent()
                    .borrow()
                    .right()
                    .clone();
                // Case 1
                if y.borrow().color == NodeColor::Red {
                    z.borrow_mut().parent_mut().borrow_mut().color = NodeColor::Black;
                    y.borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut()
                        .borrow_mut()
                        .parent_mut()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    let z_tmp = z.borrow().parent().borrow().parent().clone();
                    z = z_tmp;
                } else {
                    // Case 2
                    if &z == z.borrow().parent().borrow().right() {
                        let z_tmp = z.borrow().parent().clone();
                        z = z_tmp;
                        self.left_rotate(z.clone());
                    }
                    // Case 3
                    z.borrow_mut().parent_mut().borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut()
                        .borrow_mut()
                        .parent_mut()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    let x = z.borrow().parent().borrow().parent().clone();
                    self.right_rotate(x);
                }
            } else {
                let y = z
                    .borrow()
                    .parent()
                    .borrow()
                    .parent()
                    .borrow()
                    .left()
                    .clone();
                // Case 4
                if y.borrow().color == NodeColor::Red {
                    z.borrow_mut().parent_mut().borrow_mut().color = NodeColor::Black;
                    y.borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut()
                        .borrow_mut()
                        .parent_mut()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    let z_tmp = z.borrow().parent().borrow().parent().clone();
                    z = z_tmp;
                } else {
                    // Case 5
                    if &z == z.borrow().parent().borrow().left() {
                        let z_tmp = z.borrow().parent().clone();
                        z = z_tmp;
                        self.right_rotate(z.clone());
                    }
                    // Case 6
                    z.borrow_mut().parent_mut().borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut()
                        .borrow_mut()
                        .parent_mut()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    let x = z.borrow().parent().borrow().parent().clone();
                    self.left_rotate(x);
                }
            }
        }
        self.root.borrow_mut().color = NodeColor::Black;
    }

    fn left_rotate(&mut self, x: Rc<RefCell<Node<T>>>) {
        // Assumes that x.right != T.nil
        if !x.borrow().right().borrow().is_nil() {
            // let y = x.borrow_mut().right.take().unwrap();
            let y = x.borrow().right().clone();
            x.borrow_mut().set_right_child(y.borrow().left().clone());
            if !y.borrow().left().borrow().is_nil() {
                y.borrow_mut().left_mut().borrow_mut().set_parent(x.clone());
            }
            y.borrow_mut().set_parent(x.borrow().parent().clone());
            if x.borrow().parent().borrow().is_nil() {
                self.root = y.clone();
            } else if &x == x.borrow().parent().borrow().left() {
                x.borrow_mut()
                    .parent_mut()
                    .borrow_mut()
                    .set_left_child(y.clone());
            } else {
                x.borrow_mut()
                    .parent_mut()
                    .borrow_mut()
                    .set_right_child(y.clone());
            }

            y.borrow_mut().set_left_child(x.clone());
            x.borrow_mut().set_parent(y);
        } else {
            panic!(
                "Invariant violated. The right child of {:?} must not be T.nil.",
                x
            );
        }
    }

    fn right_rotate(&mut self, y: Rc<RefCell<Node<T>>>) {
        // Assumes that x.left != T.nil
        if !y.borrow().left().borrow().is_nil() {
            // let x = y.clone().borrow_mut().left.take().unwrap();
            let x = y.borrow().left().clone();
            y.borrow_mut().set_left_child(x.borrow().right().clone());
            if !x.borrow().right().borrow().is_nil() {
                x.borrow_mut()
                    .right_mut()
                    .borrow_mut()
                    .set_parent(y.clone());
            }
            x.borrow_mut().set_parent(y.borrow().parent().clone());
            if y.borrow().parent().borrow().is_nil() {
                self.root = x.clone();
            } else if &y.clone() == y.borrow().parent().borrow().right() {
                y.borrow_mut()
                    .parent_mut()
                    .borrow_mut()
                    .set_right_child(x.clone());
            } else {
                y.borrow_mut()
                    .parent_mut()
                    .borrow_mut()
                    .set_left_child(x.clone());
            }
            x.borrow_mut().set_right_child(y.clone());
            y.borrow_mut().set_parent(x);
        } else {
            panic!(
                "Invariant violated. The left child of {:?} must not be T.nil.",
                y
            );
        }
    }

    pub fn delete(&mut self, key: T) {
        let node = self.search(key);
        if node.is_some() {
            self.delete_node(node.as_ref().unwrap().clone())
        }
        self.length -= 1;
    }

    fn delete_node(&mut self, z: Rc<RefCell<Node<T>>>) {
        let mut y = z.clone();
        let mut y_color = y.borrow().color.clone();
        let x;
        if z.borrow().left().borrow().is_nil() {
            x = z.borrow().right().clone();
            let u = z.clone();
            let v = z.borrow().right().clone();
            self.transplant(u, v);
        } else if z.borrow().right().borrow().is_nil() {
            x = z.borrow().left().clone();
            let u = z.clone();
            let v = z.borrow().left().clone();
            self.transplant(u, v);
        } else {
            y = self
                .minimum_node(z.borrow().right().clone())
                .expect("Expected this to be set");
            y_color = y.borrow().color.clone();
            x = y.borrow().right().clone();
            if &y != z.borrow().right() {
                let u = y.clone();
                let v = y.borrow().right().clone();
                self.transplant(u, v);
                y.borrow_mut().set_right_child(z.borrow().right().clone());
                y.borrow_mut()
                    .right_mut()
                    .borrow_mut()
                    .set_parent(y.clone());
            } else {
                x.borrow_mut().set_parent(y.clone());
            }
            let u = z.clone();
            let v = y.clone();
            self.transplant(u, v);
            y.borrow_mut().set_left_child(z.borrow().left().clone());
            y.borrow_mut().left_mut().borrow_mut().set_parent(y.clone());
            y.borrow_mut().color = z.borrow().color.clone();
        }

        if y_color == NodeColor::Black {
            self.delete_fix_up(x);
        }
    }

    fn transplant(&mut self, u: Rc<RefCell<Node<T>>>, v: Rc<RefCell<Node<T>>>) {
        if u.borrow().parent().borrow().is_nil() {
            self.root = v.clone();
        } else if &u == u.borrow().parent().borrow().left() {
            u.borrow_mut()
                .parent_mut()
                .borrow_mut()
                .set_left_child(v.clone());
        } else {
            u.borrow_mut()
                .parent_mut()
                .borrow_mut()
                .set_right_child(v.clone());
        }
        v.borrow_mut().set_parent(u.borrow().parent().clone());
    }

    fn delete_fix_up(&mut self, mut x: Rc<RefCell<Node<T>>>) {
        while x != self.root && x.borrow().color == NodeColor::Black {
            if &x == x.borrow().parent().borrow().left() {
                let mut w = x.borrow().parent().borrow().right().clone();
                // Case 1
                if w.borrow().color == NodeColor::Red {
                    w.borrow_mut().color = NodeColor::Black;
                    x.borrow_mut().parent_mut().borrow_mut().color = NodeColor::Red;
                    self.left_rotate(x.borrow().parent().clone());
                    w = x.borrow().parent().borrow().right().clone();
                }

                if w.borrow().left().borrow().color == NodeColor::Black
                    && w.borrow().right().borrow().color == NodeColor::Black
                {
                    w.borrow_mut().color = NodeColor::Red;
                    let x_tmp = x.borrow().parent().clone();
                    x = x_tmp;
                } else {
                    // Case 3
                    if w.borrow_mut().right().borrow().color == NodeColor::Black {
                        w.borrow_mut().left().borrow_mut().color = NodeColor::Black;
                        w.borrow_mut().color = NodeColor::Red;
                        self.right_rotate(w.clone());
                        w = x.borrow().parent().borrow().right().clone();
                    }
                    // Case 4
                    w.borrow_mut().color = x.borrow().parent().borrow().color.clone();
                    x.borrow_mut().parent_mut().borrow_mut().color = NodeColor::Black;
                    w.borrow_mut().right_mut().borrow_mut().color = NodeColor::Black;
                    self.left_rotate(x.borrow().parent().clone());
                    x = self.root.clone();
                }
            } else {
                let mut w = x.borrow().parent().borrow().left().clone();
                // Case 5
                if w.borrow_mut().color == NodeColor::Red {
                    w.borrow_mut().color = NodeColor::Black;
                    x.borrow_mut().parent_mut().borrow_mut().color = NodeColor::Red;
                    self.right_rotate(x.borrow().parent().clone());
                    w = x.borrow().parent().borrow().left().clone();
                }
                // Case 6
                if w.borrow().right().borrow().color == NodeColor::Black
                    && w.borrow().left().borrow().color == NodeColor::Black
                {
                    w.borrow_mut().color = NodeColor::Red;
                    let x_tmp = x.borrow().parent().clone();
                    x = x_tmp;
                } else {
                    // Case 7
                    if w.borrow().left().borrow().color == NodeColor::Black {
                        w.borrow_mut().right_mut().borrow_mut().color = NodeColor::Black;
                        w.borrow_mut().color = NodeColor::Red;
                        self.left_rotate(w.clone());
                        w = x.borrow_mut().parent().borrow().left().clone();
                    }
                    // Case 8
                    w.borrow_mut().color = x.borrow().parent().borrow().color.clone();
                    x.borrow_mut().parent_mut().borrow_mut().color = NodeColor::Black;
                    w.borrow_mut().left_mut().borrow_mut().color = NodeColor::Black;
                    self.right_rotate(x.borrow().parent().clone());
                    x = self.root.clone();
                }
            }
        }
        x.borrow_mut().color = NodeColor::Black;
    }

    fn search(&self, key: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut node = self.root.clone();
        while !node.borrow().is_nil() {
            let node_key = node.borrow().key.clone();
            if node_key == key {
                return Some(node);
            } else if key < node_key {
                let node_tmp = node.borrow().left().clone();
                node = node_tmp;
            } else {
                let node_tmp = node.borrow().right().clone();
                node = node_tmp;
            }
        }
        None
    }

    pub fn clear(&mut self) {
        self.root = self.sentinel.clone();
        self.length = 0;
    }

    fn minimum_node(&self, node: Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        if node.borrow().left().borrow().is_nil() {
            return Some(node);
        }

        let mut x = node.clone();
        while !x.borrow().left().borrow().is_nil() {
            let x_tmp = x.borrow().left().clone();
            x = x_tmp;
        }
        Some(x)
    }
}
