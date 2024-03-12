use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::rc::Rc;


#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct TreeNode<T: Ord> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct Tree<T: Ord>(Option<Rc<RefCell<TreeNode<T>>>>);

impl<T: Ord> From<TreeNode<T>> for Tree<T> {
    fn from(tree_node: TreeNode<T>) -> Self {
        Tree(Some(Rc::new(RefCell::new(tree_node))))
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct BinaryTree<T: Ord> {
    root: Tree<T>,
    size: usize,
}

impl<T: Ord> TreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            left: Tree::default(),
            right: Tree::default(),
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: Tree::default(),
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, val: T) {
        todo!()
    }
}
