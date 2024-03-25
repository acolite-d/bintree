use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::{FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{null, NonNull};

#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode<T: Ord> {
    item: T,
    left: Tree<T>,
    right: Tree<T>,
    height: usize,
}

type TreePtr<T> = Option<Box<TreeNode<T>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Tree<T: Ord>(TreePtr<T>);

impl<T: Ord> TreeNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            left: Tree(None),
            right: Tree(None),
            height: 1,
        }
    }
}

struct TreeWalker<'tree, T: Ord> {
    curr_ptr: NonNull<TreeNode<T>>,
    _phantom: PhantomData<&'tree mut TreeNode<T>>
}

impl<'tree, T: Ord> TreeWalker<'tree, T> {
    fn new(node_ref: &'tree mut TreeNode<T>) -> Self {
        Self {
            curr_ptr: NonNull::from(node_ref),
            _phantom: PhantomData
        }
    }

    fn walk_to_leaf_by_val(&mut self, val: &T) -> Option<()> {
        let curr_node = unsafe { self.curr_ptr.as_mut() };

        match val.cmp(&curr_node.item) {

            Ordering::Greater => {
                if let Some(next) = curr_node.right.0.as_deref_mut() {
                    self.curr_ptr = NonNull::from(next);
                    Some(())
                } else {
                    None
                }
            }

            Ordering::Less => {
                if let Some(next) = curr_node.left.0.as_deref_mut() {
                    self.curr_ptr = NonNull::from(next);
                    Some(())
                } else {
                    None
                }
            }

            Ordering::Equal => None,
        }
    }

    fn into_inner(mut self) -> &'tree mut TreeNode<T> {
        unsafe { self.curr_ptr.as_mut() }
    }
}

impl<T: Ord> From<TreeNode<T>> for Tree<T> {
    fn from(node: TreeNode<T>) -> Self {
        Tree(Some(Box::new(node)))
    }
}

impl<T: Ord> Default for Tree<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Ord> Tree<T> {
    fn search(&self, target: &T) -> Option<&TreeNode<T>> {
        let mut tree = self;

        while let Some(node) = tree.0.as_deref() {
            match target.cmp(&node.item) {
                Ordering::Less => tree = &node.left,
                Ordering::Greater => tree = &node.right,
                Ordering::Equal => break,
            }
        }

        tree.0.as_deref()
    }

    fn next_leaf(&mut self, next_item: &T) -> &mut Tree<T> {

        let mut walker = TreeWalker::new(&mut self.0.as_deref_mut().unwrap());

        while let Some(()) = walker.walk_to_leaf_by_val(&next_item) {}

        let leaf = walker.into_inner();

        while let Some(node) = tree.0.as_deref_mut() {
            match next_item.cmp(&node.item) {
                Ordering::Greater => tree = &mut node.right,
                Ordering::Less => tree = &mut node.left,
                Ordering::Equal => break,
            }
        }

        tree
    }

    fn add_child(&mut self, new_item: T) {
        let mut walker = TreeWalker::new(&mut self.0.as_deref_mut().unwrap());

        while let Some(()) = walker.walk_to_leaf_by_val(&next_item) {}

        let leaf = walker.into_inner();

        match new_item.cmp(&new_item) {
            Ordering::Equal => {},

            Ordering::Greater => {
                leaf.right.0.insert(TreeNode::new(new_item).into());
            }

            Ordering::Less => {
                leaf.right.0.insert(TreeNode::new(new_item).into());
            }
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct BinTree<T: Ord> {
    root: Tree<T>,
    size: usize,
}

impl<T: Ord> BinTree<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            root: Tree::default(),
            size: 0,
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn clear(&mut self) {
        _ = self.root.0.take();
        self.size = 0;
    }

    #[inline]
    pub fn search(&self, target: &T) -> Option<&T> {
        self.root.search(target).map(|node| &node.item)
    }
}