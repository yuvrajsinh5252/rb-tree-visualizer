use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt::Debug;

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
        let parent = if self.parent.is_some() {
            format!("{}", self.parent.unwrap())
        } else {
            "_".to_string()
        };
        write!(
            f,
            "Node {:?} parent {} left: {} right: {} k: {:?} v: {:?}, s: {}",
            self.color, parent, left, right, self.key, self.value, self.size
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

pub struct RBTree<K, V>
where
    K: Ord,
{
    pub root: Option<usize>,
    pub nodes: Vec<Node<K, V>>,
}

impl<K, V> RBTree<K, V>
where
    K: Ord + Debug,
    V: Debug,
{
    pub fn new() -> RBTree<K, V> {
        Self {
            root: None,
            nodes: vec![],
        }
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

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn is_red(maybe_id: Option<usize>, nodes: &[Node<K, V>]) -> bool {
        maybe_id.is_some() && nodes[maybe_id.unwrap()].color.is_red()
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

        nodes[x].parent = nodes[h].parent;
        nodes[h].parent = Some(x);
        if let Some(right) = nodes[h].right {
            nodes[right].parent = Some(h);
        }

        nodes[x].size = nodes[h].size;
        nodes[h].size = Self::size(nodes[h].left, nodes) + Self::size(nodes[h].right, nodes) + 1;

        x
    }

    fn rotate_right(h: usize, nodes: &mut Vec<Node<K, V>>) -> usize {
        let x = nodes[h].left.unwrap();

        nodes[h].left = nodes[x].right;
        nodes[x].right = Some(h);
        nodes[x].color = nodes[h].color;
        nodes[h].color = Color::red();

        nodes[x].parent = nodes[h].parent;
        nodes[h].parent = Some(x);
        if let Some(left) = nodes[h].left {
            nodes[left].parent = Some(h);
        }

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
