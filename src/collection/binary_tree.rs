use std::fmt::Display;

pub type BTree<K, V> = Node<K, V>;

#[derive(Debug)]
pub struct Node<K, V: Display> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

pub trait BinaryTree<K, V> {
    fn insert(&mut self, k: K, v: V);
    fn pre_order(&self, func: fn(&K, &V));
    fn in_order(&self, func: fn(&K, &V));
    fn pos_order(&self, func: fn(&K, &V));
}

impl<K, V: Display> Node<K, V> {
    pub fn new(k: K, v: V) -> Self {
        Node {
            key: k,
            value: v,
            left: None,
            right: None,
        }
    }
}

impl<K: PartialOrd, V: Display> BinaryTree<K, V> for Node<K, V> {
    fn insert(&mut self, k: K, v: V) {
        if k < self.key {
            if let Some(ref mut left_node) = self.left {
                left_node.insert(k, v);
            } else {
                self.left = Some(Box::new(Node::new(k, v)))
            }
        } else {
            if let Some(ref mut right_node) = self.right {
                right_node.insert(k, v);
            } else {
                self.right = Some(Box::new(Node::new(k, v)))
            }
        }
    }

    fn pre_order(&self, func: fn(&K, &V)) {
        func(&self.key, &self.value);
        if let Some(ref left) = self.left {
            left.pre_order(func);
        }
        if let Some(ref right) = self.right {
            right.pre_order(func);
        }
    }

    fn in_order(&self, func: fn(&K, &V)) {
        if let Some(ref left) = self.left {
            left.in_order(func);
        }
        func(&self.key, &self.value);
        if let Some(ref right) = self.right {
            right.in_order(func);
        }
    }

    fn pos_order(&self, func: fn(&K, &V)) {
        if let Some(ref left) = self.left {
            left.pos_order(func);
        }
        if let Some(ref right) = self.right {
            right.pos_order(func);
        }
        func(&self.key, &self.value);
    }
}