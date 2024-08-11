use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node<T> {
    me: Weak<RefCell<Node<T>>>,
    children: Option<Vec<Rc<RefCell<Node<T>>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
    value: Option<T>,
}

impl<T> Node<T> {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(Node {
                children: None,
                parent: None,
                value: None,
                me: me.clone(),
            })
        })
    }

    pub fn new_with_value(value: T) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(Node {
                children: None,
                parent: None,
                value: Some(value),
                me: me.clone(),
            })
        })
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn get_value_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node<T>>>) {
        self.parent = Some(parent);
    }

    pub fn get_parent(&self) -> Option<Weak<RefCell<Node<T>>>> {
        self.parent.as_ref().map(|x| x.clone())
    }

    pub fn insert_child_value(&mut self, value: T) {
        let child = Self::new_with_value(value);
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
}

#[derive(Debug)]
pub struct Tree<T> {
    root: Rc<RefCell<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: Node::new() }
    }

    pub fn new_with_value(value: T) -> Self {
        Tree {
            root: Node::new_with_value(value),
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
