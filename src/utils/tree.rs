use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub enum Node<K, V>
where
    K: Eq + Hash,
{
    Branch(Box<Tree<K, V>>),
    Leaf(V),
}

#[derive(Debug)]
pub struct Tree<K, V>
where
    K: Eq + Hash,
{
    nodes: HashMap<K, Node<K, V>>,
}

impl<K, V> Tree<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, node: Node<K, V>) -> Option<Node<K, V>> {
        self.nodes.insert(key, node)
    }

    pub fn insert_leaf(&mut self, key: K, value: V) -> Option<Node<K, V>> {
        self.nodes.insert(key, Node::Leaf(value))
    }

    pub fn insert_branch(&mut self, key: K, value: Tree<K, V>) -> Option<Node<K, V>> {
        self.nodes.insert(key, Node::Branch(Box::new(value)))
    }

    pub fn create_branch(mut self, key: K) -> Option<Node<K, V>> {
        self.nodes
            .insert(key, Node::Branch(Box::new(Tree::<K, V>::new())))
    }

    pub fn get(&self, key: &K) -> Option<&Node<K, V>> {
        self.nodes.get(key)
    }

    pub fn get_branch(&self, key: &K) -> Option<&Box<Tree<K, V>>> {
        match self.get(key) {
            Some(Node::Branch(subtree)) => Some(subtree),
            _ => None,
        }
    }

    pub fn get_leaf(&self, key: &K) -> Option<&V> {
        match self.get(key) {
            Some(Node::Leaf(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut Node<K, V>> {
        self.nodes.get_mut(key)
    }

    pub fn get_branch_mut(&mut self, key: &K) -> Option<&mut Box<Tree<K, V>>> {
        match self.get_mut(key) {
            Some(Node::Branch(subtree)) => Some(subtree),
            _ => None,
        }
    }

    pub fn get_leaf_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.get_mut(key) {
            Some(Node::Leaf(value)) => Some(value),
            _ => None,
        }
    }
}

impl<K, V> Index<K> for Tree<K, V>
where
    K: Eq + Hash,
{
    type Output = Node<K, V>;

    fn index(&self, index: K) -> &Self::Output {
        &self.nodes[&index]
    }
}
