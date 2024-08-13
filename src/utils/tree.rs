use std::{collections::VecDeque, marker::PhantomData, rc::Rc};

pub use std::cell::{Ref, RefCell, RefMut};

pub struct Tree<'a, T> {
    children: Vec<Tree<'a, T>>,
    value: Rc<RefCell<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Tree<'a, T> {
    pub fn new(value: T) -> Self {
        Tree {
            children: Vec::new(),
            value: Rc::new(RefCell::new(value)),
            _marker: PhantomData,
        }
    }

    pub fn get_value_pointer(&'a self) -> Rc<RefCell<T>> {
        self.value.clone()
    }

    pub fn get_value(&'a self) -> Ref<'a, T> {
        self.value.borrow()
    }

    pub fn get_value_mut(&'a mut self) -> RefMut<'a, T> {
        self.value.borrow_mut()
    }

    pub fn replace(&mut self, value: T) -> T {
        self.value.replace(value)
    }

    pub fn insert_child(&mut self, child: Tree<'a, T>) {
        self.children.push(child);
    }

    pub fn insert_child_value(&mut self, value: T) {
        self.insert_child(Tree::new(value));
    }

    pub fn get_child(&self, index: usize) -> Option<&Tree<'a, T>> {
        self.children.get(index)
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Tree<'a, T>> {
        self.children.get_mut(index)
    }

    pub fn remove(&mut self, index: usize) -> Tree<T> {
        let x = self.children.remove(index);
        Tree {
            children: x.children,
            value: x.value,
            _marker: PhantomData,
        }
    }

    pub fn dfs_iter(&'a self) -> DfsTreeIterator<'a, T> {
        DfsTreeIterator::new(self)
    }

    pub fn bfs_iter(&'a self) -> BfsTreeIterator<'a, T> {
        BfsTreeIterator::new(self)
    }

    // TODO: Ergonomics, create mut iters
}

struct TreeIteratorState<'a, T> {
    tree: &'a Tree<'a, T>,
    child_index: usize,
    visited: bool,
    depth: usize,
}

impl<'a, T> TreeIteratorState<'a, T> {
    pub fn unvisited(tree: &'a Tree<'a, T>) -> Self {
        Self {
            tree,
            child_index: 0,
            visited: false,
            depth: 0,
        }
    }

    pub fn visited(tree: &'a Tree<'a, T>) -> Self {
        Self {
            tree,
            child_index: 0,
            visited: true,
            depth: 0,
        }
    }

    pub fn at_index(tree: &'a Tree<'a, T>, child_index: usize) -> Self {
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

pub struct DfsTreeIterator<'a, T> {
    iter_stack: Vec<TreeIteratorState<'a, T>>,
    max_depth: usize,
}

impl<'a, T> DfsTreeIterator<'a, T> {
    fn new(tree: &'a Tree<T>) -> Self {
        Self {
            iter_stack: vec![TreeIteratorState::unvisited(tree)],
            max_depth: 0,
        }
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }
}

impl<'a, T> Iterator for DfsTreeIterator<'a, T> {
    type Item = Ref<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.iter_stack.pop() {
            if !state.visited {
                let value = state.tree.get_value();
                self.iter_stack
                    .push(TreeIteratorState::visited(state.tree).with_depth(state.depth));
                return Some(value);
            }

            if state.depth >= self.max_depth && self.max_depth > 0 {
                continue;
            }

            if let Some(child) = state.tree.get_child(state.child_index) {
                self.iter_stack.push(
                    TreeIteratorState::at_index(state.tree, state.child_index + 1)
                        .with_depth(state.depth),
                );
                let value = child.get_value();
                self.iter_stack
                    .push(TreeIteratorState::visited(child).with_depth(state.depth + 1));
                return Some(value);
            }
        }

        None
    }
}
pub struct BfsTreeIterator<'a, T> {
    iter_stack: VecDeque<TreeIteratorState<'a, T>>,
    max_depth: usize,
}

impl<'a, T> BfsTreeIterator<'a, T> {
    fn new(tree: &'a Tree<T>) -> Self {
        Self {
            iter_stack: VecDeque::from([TreeIteratorState::unvisited(tree)]),
            max_depth: 0,
        }
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }
}

impl<'a, T> Iterator for BfsTreeIterator<'a, T> {
    type Item = Ref<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.iter_stack.pop_front() {
            if !state.visited {
                let value = state.tree.get_value();
                self.iter_stack
                    .push_back(TreeIteratorState::visited(state.tree).with_depth(state.depth));
                return Some(value);
            }

            if state.depth >= self.max_depth && self.max_depth > 0 {
                continue;
            }

            if let Some(child) = state.tree.get_child(state.child_index) {
                self.iter_stack.push_front(
                    TreeIteratorState::at_index(state.tree, state.child_index + 1)
                        .with_depth(state.depth),
                );
                let value = child.get_value();
                self.iter_stack
                    .push_back(TreeIteratorState::visited(child).with_depth(state.depth + 1));
                return Some(value);
            }
        }

        None
    }
}
