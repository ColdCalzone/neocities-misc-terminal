use std::{
    collections::VecDeque,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

pub use std::cell::{Ref, RefCell, RefMut};

#[derive(Debug)]
pub struct SendTree<'a, T> {
    children: Vec<SendTree<'a, T>>,
    value: Arc<RwLock<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> SendTree<'a, T> {
    pub fn new(value: T) -> Self {
        SendTree {
            children: Vec::new(),
            value: Arc::new(RwLock::new(value)),
            _marker: PhantomData,
        }
    }

    pub fn get_value(&'a self) -> Arc<RwLock<T>> {
        self.value.clone()
    }

    pub fn insert_child(&mut self, child: SendTree<'a, T>) {
        self.children.push(child);
    }

    pub fn with_child(mut self, child: SendTree<'a, T>) -> SendTree<'a, T> {
        self.insert_child(child);
        self
    }

    pub fn insert_child_value(&mut self, value: T) {
        self.insert_child(SendTree::new(value));
    }

    pub fn get_child(&self, index: usize) -> Option<&SendTree<'a, T>> {
        self.children.get(index)
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut SendTree<'a, T>> {
        self.children.get_mut(index)
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn count(&self) -> usize {
        self.children.len()
    }

    pub fn remove(&mut self, index: usize) -> SendTree<T> {
        let x = self.children.remove(index);
        SendTree {
            children: x.children,
            value: x.value,
            _marker: PhantomData,
        }
    }

    pub fn dfs_iter(&'a self) -> DfsSendTreeIterator<'a, T> {
        DfsSendTreeIterator::new(self)
    }

    pub fn bfs_iter(&'a self) -> BfsSendTreeIterator<'a, T> {
        BfsSendTreeIterator::new(self)
    }

    // TODO: Ergonomics, create mut iters
}

struct SendTreeIteratorState<'a, T> {
    tree: &'a SendTree<'a, T>,
    child_index: usize,
    visited: bool,
    depth: usize,
}

impl<'a, T> SendTreeIteratorState<'a, T> {
    pub fn unvisited(tree: &'a SendTree<'a, T>) -> Self {
        Self {
            tree,
            child_index: 0,
            visited: false,
            depth: 0,
        }
    }

    pub fn visited(tree: &'a SendTree<'a, T>) -> Self {
        Self {
            tree,
            child_index: 0,
            visited: true,
            depth: 0,
        }
    }

    pub fn at_index(tree: &'a SendTree<'a, T>, child_index: usize) -> Self {
        Self {
            tree,
            child_index,
            visited: true,
            depth: 0,
        }
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
}

pub struct DfsSendTreeIterator<'a, T> {
    iter_stack: Vec<SendTreeIteratorState<'a, T>>,
    max_depth: usize,
    skip_root: bool,
}

impl<'a, T> DfsSendTreeIterator<'a, T> {
    fn new(tree: &'a SendTree<T>) -> Self {
        Self {
            iter_stack: vec![SendTreeIteratorState::unvisited(tree)],
            max_depth: 0,
            skip_root: false,
        }
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn skip_root(mut self) -> Self {
        self.skip_root = true;
        self
    }
}

impl<'a, T> Iterator for DfsSendTreeIterator<'a, T> {
    type Item = &'a SendTree<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.iter_stack.pop() {
            if !state.visited && !self.skip_root {
                self.iter_stack
                    .push(SendTreeIteratorState::visited(state.tree).with_depth(state.depth));
                return Some(&state.tree);
            }

            if state.depth >= self.max_depth && self.max_depth > 0 {
                continue;
            }

            if let Some(child) = state.tree.get_child(state.child_index) {
                self.iter_stack.push(
                    SendTreeIteratorState::at_index(state.tree, state.child_index + 1)
                        .with_depth(state.depth),
                );
                self.iter_stack
                    .push(SendTreeIteratorState::visited(child).with_depth(state.depth + 1));
                return Some(child);
            }
        }

        None
    }
}
pub struct BfsSendTreeIterator<'a, T> {
    iter_stack: VecDeque<SendTreeIteratorState<'a, T>>,
    max_depth: usize,
    skip_root: bool,
}

impl<'a, T> BfsSendTreeIterator<'a, T> {
    fn new(tree: &'a SendTree<T>) -> Self {
        Self {
            iter_stack: VecDeque::from([SendTreeIteratorState::unvisited(tree)]),
            max_depth: 0,
            skip_root: false,
        }
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn skip_root(mut self) -> Self {
        self.skip_root = true;
        self
    }
}

impl<'a, T> Iterator for BfsSendTreeIterator<'a, T> {
    type Item = &'a SendTree<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.iter_stack.pop_front() {
            if !state.visited && !self.skip_root {
                self.iter_stack
                    .push_back(SendTreeIteratorState::visited(state.tree).with_depth(state.depth));
                return Some(state.tree);
            }

            if state.depth >= self.max_depth && self.max_depth > 0 {
                continue;
            }

            if let Some(child) = state.tree.get_child(state.child_index) {
                self.iter_stack.push_front(
                    SendTreeIteratorState::at_index(state.tree, state.child_index + 1)
                        .with_depth(state.depth),
                );
                self.iter_stack
                    .push_back(SendTreeIteratorState::visited(child).with_depth(state.depth + 1));
                return Some(child);
            }
        }

        None
    }
}
