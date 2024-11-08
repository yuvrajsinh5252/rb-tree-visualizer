use web_sys::console;

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    color: Color,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    parent: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T, color: Color) -> Self {
        Node {
            value,
            color,
            left: None,
            right: None,
            parent: None,
        }
    }
}

pub struct RedBlackTree<T> {
    pub root: Option<Box<Node<T>>>,
}
impl<T: Ord + std::fmt::Debug> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, Color::Red));

        if self.root.is_none() {
            self.root = Some(new_node);
            self.root.as_mut().unwrap().color = Color::Black;
        } else {
            let mut root = self.root.take().unwrap();
            self.insert_node(&mut root, new_node);
            self.root = Some(root);
        }
    }

    fn insert_node(&mut self, root: &mut Box<Node<T>>, new_node: Box<Node<T>>) {
        if new_node.value < root.value {
            if root.left.is_none() {
                root.left = Some(new_node);
                root.left.as_mut().unwrap().parent = Some(root.as_mut() as *mut _);
                self.fix_insert(root.left.as_mut().unwrap());
            } else {
                self.insert_node(root.left.as_mut().unwrap(), new_node);
            }
        } else {
            if root.right.is_none() {
                root.right = Some(new_node);
                root.right.as_mut().unwrap().parent = Some(root.as_mut() as *mut _);
                self.fix_insert(root.right.as_mut().unwrap());
            } else {
                self.insert_node(root.right.as_mut().unwrap(), new_node);
            }
        }
    }

    fn fix_insert(&mut self, node: &mut Box<Node<T>>) {
        // Fixing the tree after insertion
        // This is a placeholder for the actual fix-up logic
    }
}
