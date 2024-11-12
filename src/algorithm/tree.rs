#![allow(dead_code)]

use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Node<K, V> {
    pub value: V,
    pub key: K,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub size: usize,
    pub color: Color,
    pub parent: Option<usize>,
}

#[derive(Copy, Clone)]
pub struct Color {
    color: bool,
}

impl Color {
    const RED: bool = true;
    const BLACK: bool = false;
    fn red() -> Color {
        Color { color: Color::RED }
    }
    fn black() -> Color {
        Color {
            color: Color::BLACK,
        }
    }
    fn is_red(self) -> bool {
        self.color == Color::RED
    }
    fn flip(&mut self) {
        self.color = !self.color;
    }
}

impl<K, V> Debug for Node<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let left = if self.left.is_some() {
            format!("{}", self.left.unwrap())
        } else {
            "_".to_string()
        };
        let right = if self.right.is_some() {
            format!("{}", self.right.unwrap())
        } else {
            "_".to_string()
        };
        let paren = if self.parent.is_some() {
            format!("{}", self.parent.unwrap())
        } else {
            "_".to_string()
        };
        write!(
            f,
            "Node {:?} parent {} left: {} right: {} k: {:?} v: {:?}, s: {}",
            self.color, paren, left, right, self.key, self.value, self.size
        )
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_red() {
            write!(f, "RED")
        } else {
            write!(f, "BLACK")
        }
    }
}

#[derive(Clone)]
pub struct RBTree<K, V>
where
    K: Ord,
{
    pub root: Option<usize>,
    pub nodes: Vec<Node<K, V>>,
}

struct DeleteResult {
    child: Option<usize>,
    moved_node: usize,
    moved_node_new_id: usize,
}

impl DeleteResult {
    fn translate(&self, id: usize) -> usize {
        if self.moved_node == id {
            self.moved_node_new_id
        } else {
            id
        }
    }

    fn set_child(mut self, id: usize) -> DeleteResult {
        self.child = Some(self.translate(id));
        self
    }
}

impl<K, V> RBTree<K, V>
where
    K: Ord + Debug,
    V: Debug,
{
    pub fn new() -> RBTree<K, V> {
        Self::default()
    }

    pub fn clear_tree(&mut self) {
        self.root = None;
        self.nodes.clear();
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut maybe_id = self.root;
        while let Some(id) = maybe_id {
            let node = &self.nodes[id];
            match key.cmp(&node.key) {
                Ordering::Equal => return Some(&node.value),
                Ordering::Less => maybe_id = node.left,
                Ordering::Greater => maybe_id = node.right,
            }
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.root = Self::put(self.root, None, key, value, &mut self.nodes);
        self.nodes[self.root.unwrap()].color = Color::black();
    }

    pub fn len(&self) -> usize {
        Self::size(self.root, &self.nodes)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn is_balanced(&self) -> bool {
        let mut black = 0;
        let mut node = self.root;
        while node.is_some() {
            if !Self::is_red(node, &self.nodes) {
                black += 1;
            }
            node = self.nodes[node.unwrap()].left;
        }
        self.node_balanced(self.root, black)
    }

    fn node_balanced(&self, maybe_id: Option<usize>, black: i32) -> bool {
        if let Some(id) = maybe_id {
            let diff = if self.nodes[id].color.is_red() { 0 } else { -1 };
            self.node_balanced(self.nodes[id].left, black + diff)
                && self.node_balanced(self.nodes[id].right, black + diff)
        } else {
            black == 0
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn delete(&mut self, key: &K) {
        if !self.contains(key) {
            return;
        }

        {
            let root = self.root.unwrap();
            if Self::is_red(self.nodes[root].left, &self.nodes)
                && Self::is_red(self.nodes[root].right, &self.nodes)
            {
                self.nodes[root].color = Color::red();
            }
        }
        let DeleteResult { child: root, .. } =
            Self::delete_node(self.root.unwrap(), key, &mut self.nodes);
        self.root = root;

        if !self.is_empty() {
            self.nodes[self.root.unwrap()].color = Color::black();
        }
    }

    pub fn print(&self) {
        Self::print_node(self.root, 0, &self.nodes);
    }

    fn print_node(maybe_id: Option<usize>, depth: usize, nodes: &[Node<K, V>]) {
        let indent = "     ".repeat(depth);
        if let Some(id) = maybe_id {
            println!("{} {:?}", indent, nodes[id]);
            Self::print_node(nodes[id].left, depth + 1, nodes);
            Self::print_node(nodes[id].right, depth + 1, nodes);
        } else {
            println!("{} None", indent);
        }
    }

    fn swap_delete_min(
        mut child: usize,
        parent: usize,
        nodes: &mut Vec<Node<K, V>>,
    ) -> DeleteResult {
        if let Some(left) = nodes[child].left {
            if Self::two_left_black(child, nodes) {
                child = Self::move_red_left(child, nodes);
            }
            let result = Self::swap_delete_min(left, parent, nodes);
            child = result.translate(child);
            nodes[child].left = result.child;
            child = Self::balance(child, nodes);
            result.set_child(child)
        } else {
            nodes[child].parent = nodes[parent].parent;
            nodes[child].color = nodes[parent].color;
            nodes[child].left = nodes[parent].left;
            nodes[child].right = nodes[parent].right;
            nodes[child].size = nodes[parent].size;
            nodes.swap(child, parent);
            Self::remove(parent, nodes)
        }
    }

    fn two_left_black(id: usize, nodes: &[Node<K, V>]) -> bool {
        let left = nodes[id].left;
        !Self::is_red(left, nodes) && !Self::is_red(nodes[left.unwrap()].left, nodes)
    }

    fn delete_node(mut id: usize, key: &K, nodes: &mut Vec<Node<K, V>>) -> DeleteResult {
        let result: DeleteResult;

        if key < &nodes[id].key {
            if Self::two_left_black(id, nodes) {
                id = Self::move_red_left(id, nodes);
            }
            result = Self::delete_node(nodes[id].left.unwrap(), key, nodes);
            id = result.translate(id);
            nodes[id].left = result.child;
        } else {
            if Self::is_red(nodes[id].left, nodes) {
                id = Self::rotate_right(id, nodes);
            }
            if key.cmp(&nodes[id].key) == Ordering::Equal && nodes[id].right.is_none() {
                // TODO: Remove from vector!
                // nodes.remove(id);
                return Self::remove(id, nodes);
            }
            // By now we've already proven that Node(id).right is Some.
            // Therefore we are safe to unwrap Node(id).right.
            let right = nodes[id].right;
            if !Self::is_red(right, nodes) && !Self::is_red(nodes[right.unwrap()].left, nodes) {
                id = Self::move_red_right(id, nodes);
            }
            // This is the node to remove.
            // We'll replace its values with those from the minimum
            // key to the right (the next greatest key from this one).
            if key.cmp(&nodes[id].key) == Ordering::Equal {
                result = Self::swap_delete_min(nodes[id].right.unwrap(), id, nodes);
            } else {
                result = Self::delete_node(nodes[id].right.unwrap(), key, nodes);
            }
            id = result.translate(id);
            nodes[id].right = result.child;
        }
        result.set_child(Self::balance(id, nodes))
    }

    fn balance(mut id: usize, nodes: &mut Vec<Node<K, V>>) -> usize {
        if Self::is_red(nodes[id].right, nodes) {
            id = Self::rotate_left(id, nodes);
        }
        let left = nodes[id].left;
        if Self::is_red(left, nodes) && Self::is_red(nodes[left.unwrap()].left, nodes) {
            id = Self::rotate_right(id, nodes);
        }
        nodes[id].size = 1 + Self::size(nodes[id].left, nodes) + Self::size(nodes[id].right, nodes);
        Self::maybe_flip(id, nodes);
        id
    }

    fn maybe_flip(id: usize, nodes: &mut Vec<Node<K, V>>) {
        if let Some(left) = nodes[id].left {
            if let Some(right) = nodes[id].right {
                if nodes[left].color.is_red() && nodes[right].color.is_red() {
                    Self::flip_colors(id, left, right, nodes);
                }
            }
        }
    }

    /// This only happens when node `id` has two consecutive black left children.
    /// Black color only happens on the left when the right is present.
    /// I don't quite understand why we can assume that node `id` is red.
    fn move_red_left(mut id: usize, nodes: &mut Vec<Node<K, V>>) -> usize {
        Self::flip_colors(id, nodes[id].left.unwrap(), nodes[id].right.unwrap(), nodes);
        if Self::is_red(nodes[nodes[id].right.unwrap()].left, nodes) {
            nodes[id].right = Some(Self::rotate_right(nodes[id].right.unwrap(), nodes));
            id = Self::rotate_left(id, nodes);
            Self::flip_colors(id, nodes[id].left.unwrap(), nodes[id].right.unwrap(), nodes);
        }
        id
    }

    fn move_red_right(mut id: usize, nodes: &mut Vec<Node<K, V>>) -> usize {
        let left = nodes[id].left.unwrap();
        Self::flip_colors(id, left, nodes[id].right.unwrap(), nodes);
        if Self::is_red(nodes[left].left, nodes) {
            id = Self::rotate_right(id, nodes);
            Self::flip_colors(id, nodes[id].left.unwrap(), nodes[id].right.unwrap(), nodes);
        }
        id
    }

    fn is_red(maybe_id: Option<usize>, nodes: &[Node<K, V>]) -> bool {
        maybe_id.is_some() && nodes[maybe_id.unwrap()].color.is_red()
    }

    fn min(mut id: usize, nodes: &[Node<K, V>]) -> usize {
        while let Some(left) = nodes[id].left {
            id = left;
        }
        id
    }

    fn put(
        maybe_id: Option<usize>,
        parent: Option<usize>,
        key: K,
        value: V,
        nodes: &mut Vec<Node<K, V>>,
    ) -> Option<usize> {
        if let Some(mut id) = maybe_id {
            let cmp = key.cmp(&nodes[id].key);
            match cmp {
                Ordering::Less => {
                    nodes[id].left = Self::put(nodes[id].left, Some(id), key, value, nodes);
                }
                Ordering::Greater => {
                    nodes[id].right = Self::put(nodes[id].right, Some(id), key, value, nodes);
                }
                Ordering::Equal => {
                    nodes[id].value = value;
                }
            }
            nodes[id].size =
                Self::size(nodes[id].left, nodes) + Self::size(nodes[id].right, nodes) + 1;

            if Self::is_red(nodes[id].right, nodes) && !Self::is_red(nodes[id].left, nodes) {
                id = Self::rotate_left(id, nodes);
            }

            if Self::is_red(nodes[id].left, nodes)
                && Self::is_red(nodes[nodes[id].left.unwrap()].left, nodes)
            {
                id = Self::rotate_right(id, nodes);
            }

            if Self::is_red(nodes[id].left, nodes) && Self::is_red(nodes[id].right, nodes) {
                Self::flip_colors(id, nodes[id].left.unwrap(), nodes[id].right.unwrap(), nodes);
            }

            Some(id)
        } else {
            let the_id = nodes.len();
            nodes.push(Node {
                key,
                value,
                parent,
                size: 1,
                left: None,
                right: None,
                color: Color::red(),
            });
            Some(the_id)
        }
    }

    fn flip_colors(base: usize, left: usize, right: usize, nodes: &mut Vec<Node<K, V>>) {
        nodes[base].color.flip();
        nodes[left].color.flip();
        nodes[right].color.flip();
    }

    fn rotate_left(h: usize, nodes: &mut Vec<Node<K, V>>) -> usize {
        let x = nodes[h].right.unwrap();

        nodes[h].right = nodes[x].left;
        nodes[x].left = Some(h);
        nodes[x].color = nodes[h].color;
        nodes[h].color = Color::red();

        // fix parents
        nodes[x].parent = nodes[h].parent;
        nodes[h].parent = Some(x);
        if let Some(right) = nodes[h].right {
            nodes[right].parent = Some(h);
        }

        // fix size
        nodes[x].size = nodes[h].size;
        nodes[h].size = Self::size(nodes[h].left, nodes) + Self::size(nodes[h].right, nodes) + 1;

        x
    }

    fn remove(id: usize, nodes: &mut Vec<Node<K, V>>) -> DeleteResult {
        let other = nodes.len() - 1;
        nodes.swap(id, other);
        if let Some(parent) = nodes[id].parent {
            let parent_node = nodes.get_mut(parent).unwrap();
            if parent_node.left.is_some() && parent_node.left.unwrap() == other {
                parent_node.left = Some(id);
            } else {
                parent_node.right = Some(id);
            }
        }
        nodes.pop();
        DeleteResult {
            child: None,
            moved_node: other,
            moved_node_new_id: id,
        }
    }

    fn rotate_right(h: usize, nodes: &mut Vec<Node<K, V>>) -> usize {
        let x = nodes[h].left.unwrap();

        nodes[h].left = nodes[x].right;
        nodes[x].right = Some(h);
        nodes[x].color = nodes[h].color;
        nodes[h].color = Color::red();

        // fix parents
        nodes[x].parent = nodes[h].parent;
        nodes[h].parent = Some(x);
        if let Some(left) = nodes[h].left {
            nodes[left].parent = Some(h);
        }

        // fix size
        nodes[x].size = nodes[h].size;
        nodes[h].size = Self::size(nodes[h].left, nodes) + Self::size(nodes[h].right, nodes) + 1;

        x
    }

    fn size(maybe_id: Option<usize>, nodes: &[Node<K, V>]) -> usize {
        if let Some(id) = maybe_id {
            nodes[id].size
        } else {
            0
        }
    }

    pub fn keys(&self) -> Vec<&K> {
        self.nodes.iter().map(|node| &node.key).collect::<Vec<&K>>()
    }
}

impl<K, V> Default for RBTree<K, V>
where
    K: Ord + Debug,
    V: Debug,
{
    fn default() -> Self {
        Self {
            root: None,
            nodes: vec![],
        }
    }
}
