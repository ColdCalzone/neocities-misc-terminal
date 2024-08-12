use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node<T> {
    me: Weak<RefCell<Node<T>>>,
    children: Option<Vec<Rc<RefCell<Node<T>>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
    value: T,
}

impl<'a, T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(Node {
                children: None,
                parent: None,
                value,
                me: me.clone(),
            })
        })
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    pub fn get_value_owned(self) -> T {
        self.value
    }

    pub fn get_value(&'a self) -> &'a T {
        &self.value
    }

    pub fn get_value_mut(&'a mut self) -> &'a mut T {
        &mut self.value
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node<T>>>) {
        self.parent = Some(parent);
    }

    pub fn get_parent(&self) -> Option<Weak<RefCell<Node<T>>>> {
        self.parent.as_ref().map(|x| x.clone())
    }

    pub fn insert_child_value(&mut self, value: T) {
        let child = Self::new(value);
        self.graft(child);
    }

    pub fn graft(&mut self, other: Rc<RefCell<Node<T>>>) {
        if self.children.is_none() {
            self.children = Some(Vec::new());
        }
        if let Some(children) = &mut self.children {
            (*other).borrow_mut().set_parent(self.me.clone());
            children.push(other);
        }
    }

    pub fn dfs_iter(&'a self) -> DfsTreeIterator<'a, T> {
        DfsTreeIterator::from_node(self)
    }
}

pub struct DfsTreeIterator<'a, T> {
    iter_stack: Vec<DfsTreeIterator<'a, T>>,
    _marker: PhantomData<&'a T>, // You stupid son of a bitch.
}

impl<'a, T> Iterator for DfsTreeIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {}
}

impl<'a, T> DfsTreeIterator<'a, T> {
    fn from_node(node: &Node<T>) -> DfsTreeIterator<'a, T> {
        todo!();
    }
}

#[derive(Debug)]
pub struct Tree<T> {
    root: Rc<RefCell<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new(root_value: T) -> Self {
        Tree {
            root: Node::new(root_value),
        }
    }

    pub fn get_root(&self) -> Ref<Node<T>> {
        (*self.root).borrow()
    }

    pub fn get_root_mut(&mut self) -> RefMut<Node<T>> {
        (*self.root).borrow_mut()
    }
}

// impl<T> DerefMut for Tree<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         Rc::new(self.get_root_mut())
//     }
// }
// impl<T> IntoIterator for Tree<T>
// where
//     K: Eq + Hash,
// {
//     type Item = Node<T>;
//     fn into_iter(self) -> Self::IntoIter {}
// }
