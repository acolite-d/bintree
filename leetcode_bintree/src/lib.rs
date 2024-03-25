use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use std::ptr;

#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode<T: Ord> {
    item: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Ord> TreeNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            left: Tree::default(),
            right: Tree::default(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Tree<T: Ord>(Option<Rc<RefCell<TreeNode<T>>>>);

impl<T: Ord> Default for Tree<T> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Ord> From<TreeNode<T>> for Tree<T> {
    fn from(tree_node: TreeNode<T>) -> Self {
        Tree(Some(Rc::new(RefCell::new(tree_node))))
    }
}

impl<T: Ord> Tree<T> {
    fn take(&mut self) -> Self {
        Tree(self.0.take())
    }
}

impl<'tree, T: Ord> Tree<T> {
    fn search(&'tree self, target: &T) -> Option<Ref<'tree, TreeNode<T>>> {
        let s = self.0
            .as_deref()
            .map(|cell| cell.borrow());

        let next = {
            match s.as_deref() {
                Some(node_ref) => node_ref.left.0.as_deref().map(|cell| cell.borrow()),
                None => None
            }
        };

        next
    }

    fn add_child(&mut self, new_item: T) {
        // let mut tree = self;

        // while let Some(node) = tree.0.as_deref().map(|refcell| refcell.borrow_mut()) {
        //     match new_item.cmp(&node.item) {
        //         Ordering::Less => tree = &mut node.left,
        //         Ordering::Greater => tree = &mut node.right,
        //         Ordering::Equal => {
        //             return;
        //         }
        //     }
        // }

        // *tree = TreeNode::new(new_item).into();
    }

    // fn remove_leftmost_child(&mut self) -> Option<Rc<RefCell<TreeNode<T>>>> {
    //     let mut tree = self;
    //     let mut leftmost = Tree(None);

    //     let pruned = { 
    //         loop {

    //             match tree.0.as_deref().map(|cell| cell.borrow_mut()).as_deref_mut() {

    //                 None => break,

    //                 Some(TreeNode {
    //                     left: Tree(None),
    //                     right: Tree(None),
    //                     ..
    //                 }) => leftmost = tree.take(),

    //                 Some(TreeNode {
    //                     left: Tree(None),
    //                     right: right @ Tree(Some(rn)),
    //                     ..
    //                 }) => {
    //                     let moved_right = right.take();
    //                     let leftmost = mem::replace(tree, moved_right);
    //                     break;
    //                 }

    //                 Some(node @ TreeNode {
    //                     left: Tree(Some(_)),
    //                     ..
    //                 }) => tree = &mut node.left
    //             }

    //         } 
    //         leftmost 
    //     };

    //     pruned.0
    // }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct BinaryTree<T: Ord> {
    root: Tree<T>,
    size: usize,
}

impl<T: Ord> BinaryTree<T> {
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
    pub fn search(&self, target: &T) -> Option<Ref<'_, T>> {
        self.root.search(target).map(|node_ref| Ref::map(node_ref, |n| &n.item))
    }

    #[inline]
    pub fn insert(&mut self, new_item: T) {
        self.root.add_child(new_item);
        self.size += 1;
    }

    // #[inline]
    // pub fn remove_inorder(&mut self) -> Option<T> {
    //     let removed = self.root.remove_leftmost_child().map(|n| {
    //         self.size -= 1;
    //         n.item
    //     });

    //     removed
    // }
}
