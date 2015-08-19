use std::boxed::Box;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
struct Node<K, V> where K: Ord + Clone, V: Clone {
    key: Box<K>,
    val: Box<V>,
    color: bool,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}


impl<K, V> Node<K, V>  where K: Ord + Clone, V: Clone{
    fn new(k: K, v: V) -> Node<K, V> {
        Node {
            key: Box::new(k),
            val: Box::new(v),
            color: true,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LLRBTree<K, V> where K: Ord + Clone, V: Clone {
    root: Option<Box<Node<K, V>>>,
}

// public
impl<K, V> LLRBTree<K, V> where K: Ord + Clone, V: Clone{
    pub fn new() -> LLRBTree<K, V> {
        LLRBTree {
            root: None,
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.root = LLRBTree::_insert(self.root.clone(), k, v);
        if let Some(ref mut r) = self.root {
            r.color = false;
        }
    }

    pub fn search(&self, k: K) -> Option<V> {
        let mut x = &self.root;
        while let &Some(ref node) = x {
            match (*node.key).cmp(&k) {
                Ordering::Equal => return Some(*node.val.clone()),
                Ordering::Less => x = &node.right,
                Ordering::Greater => x = &node.left,
            }
        }
        None
    }
}

// private
impl<K, V> LLRBTree<K, V> where K: Ord + Clone, V: Clone{
    fn _insert(h: Option<Box<Node<K, V>>>, k: K, v: V) -> Option<Box<Node<K, V>>> {
        match h {
            None => Some(Box::new(Node::<K, V>::new(k, v))),
            Some(boxed_h) => {
                let mut node = *boxed_h;
                if LLRBTree::_should_flip_color(&node) {
                    LLRBTree::_flip_color(&mut node);
                }
                match (*node.key).cmp(&k) {
                    Ordering::Equal => node.val = Box::new(v),
                    Ordering::Less => node.right = LLRBTree::_insert(node.right, k, v),
                    Ordering::Greater => node.left = LLRBTree::_insert(node.left, k, v),
                }

                if LLRBTree::_should_rotate_left(&node.left, &node.right) {
                    node = LLRBTree::_rotate_left(node);
                }

                if LLRBTree::_should_rotate_right(&node.left) {
                    node = LLRBTree::_rotate_right(node);
                }

                if LLRBTree::_should_flip_color(&node) {
                    LLRBTree::_flip_color(&mut node);
                }

                Some(Box::new(node))
            },
        }
    }

    fn _should_flip_color(node: &Node<K, V>) -> bool {
        match (&node.left, &node.right) {
            (&Some(ref l), &Some(ref r)) => {
                if l.color && r.color {
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    fn _flip_color(h: &mut Node<K, V>) {
        h.color = !h.color;
        match h.left {
            Some(ref mut left) => left.color = !left.color,
            _ => unreachable!(),
        }
        match h.right {
            Some(ref mut right) => right.color = !right.color,
            _ => unreachable!(),
        }
    }

    fn _should_rotate_left(left: &Option<Box<Node<K, V>>>, right: &Option<Box<Node<K, V>>>) -> bool {
        match (right, left) {
            (&Some(ref r), &None) => r.color,
            (&Some(ref r), &Some(ref l)) => r.color && !l.color,
            _ => false,
        }
    }

    fn _should_rotate_right(left: &Option<Box<Node<K, V>>>) -> bool {
        match left {
            &None => false,
            &Some(ref l) => {
                match l.left {
                    None => false,
                    Some(ref ll) => l.color && ll.color,
                }
            }
        }
    }

    fn _rotate_left(node_h: Node<K, V>) -> Node<K, V> {
        let Node{ color, right, ..} = node_h;
        let node_x = *right.unwrap();
        let new_h = Node {
            key: node_h.key,
            val: node_h.val,
            color: true,
            left: node_h.left,
            right: node_x.left,
        };

        Node {
            key: node_x.key,
            val: node_x.val,
            color: color,
            left: Some(Box::new(new_h)),
            right: node_x.right,
        }
    }

    fn _rotate_right(node_h: Node<K, V>) -> Node<K, V> {
        let Node{ color, left, ..} = node_h;
        let node_x = *left.unwrap();
        let new_h = Node {
            key: node_h.key,
            val: node_h.val,
            color: true,
            left: node_x.right,
            right: node_h.right,
        };

        Node {
            key: node_x.key,
            val: node_x.val,
            color: color,
            left: node_x.left,
            right: Some(Box::new(new_h)),
        }
    }
}

#[test]
fn test1() {
    let mut t = LLRBTree::<usize, usize>::new();
    t.insert(5,1);
    t.insert(6,2);
    assert_eq!(1, t.search(5).unwrap());
    assert_eq!(2, t.search(6).unwrap());
    assert_eq!(None, t.search(9));
}

#[test]
fn test2() {
    let mut t = LLRBTree::<String, char>::new();
    t.insert("Foo".to_string(),'f');
    t.insert("Bar".to_string(),'b');
    t.insert("Quux".to_string(),'q');
    t.insert("fooz".to_string(),'F');
    assert_eq!('b', t.search("Bar".to_string()).unwrap());
    assert_eq!('q', t.search("Quux".to_string()).unwrap());
    assert_eq!(None, t.search("OOO".to_string()));
}