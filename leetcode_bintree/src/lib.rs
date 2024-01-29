use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::rc::Rc;

type TreeLink<T> = Rc<RefCell<Option<TreeNode<T>>>>;

macro_rules! alloc_node {
    () => {
        Rc::new(RefCell::new(None))
    };

    ($val:expr) => {
        Rc::new(RefCell::new(TreeNode::new($val)))
    };
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct TreeNode<T: Ord> {
    val: T,
    left: TreeLink<T>,
    right: TreeLink<T>,
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct BinaryTree<T: Ord> {
    root: TreeLink<T>,
    size: usize,
}

impl<T: Ord> TreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            left: alloc_node!(),
            right: alloc_node!(),
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: alloc_node!(),
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, val: T) {
        todo!()
        // let curr_node = self.root.borrow();

        // while let Some(next) = curr_node.as_ref() {
        //     curr_node = next.left;
        // }
    }
}
