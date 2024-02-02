#[allow(unused)]
use std::cmp::Ordering;
#[allow(unused)]
use std::iter::{FromIterator, IntoIterator};
#[allow(unused)]
use std::{mem, ptr};


#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode<T: Ord> {
    item: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Tree<T: Ord>(Option<Box<TreeNode<T>>>);

impl<T: Ord> TreeNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            left: Tree(None),
            right: Tree(None),
        }
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

    fn add_child(&mut self, new_item: T) {
        match self.0.as_deref_mut() {
            None => *self = TreeNode::new(new_item).into(),

            Some(tree) => {
                match tree.item.cmp(&new_item) {
                    Ordering::Less => tree.right.add_child(new_item),
                    Ordering::Greater => tree.left.add_child(new_item),
                    Ordering::Equal => { },
                };
            }
        }
    }

    #[allow(unused)]
    fn remove_leftmost_child(&mut self) -> Option<Box<TreeNode<T>>> {
        let pruned = match self.0.as_deref_mut() {
            None => None,

            Some(TreeNode {left: l @ Tree(Some(_)), ..}) => {
                l.remove_leftmost_child()
            }

            Some(TreeNode {left: Tree(None), right: Tree(None), ..}) => {
                self.0.take()
            }

            Some(TreeNode {left: Tree(None), right: r @ Tree(Some(_)), ..}) => {
                let right_child = r.0.take();
                mem::replace(&mut self.0, right_child)
            }
        };

        pruned
    }

}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct BinTree<T: Ord> {
    root: Tree<T>,
    size: usize,
}

impl<T> BinTree<T>
where
    T: Ord,
{
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
    pub fn insert(&mut self, new_item: T) {
        self.root.add_child(new_item);
        self.size += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let popped = self.root.remove_leftmost_child().map(|n| n.item);
        self.size -= 1;
        popped
    }
}

pub struct InorderIntoIter<T: Ord>(BinTree<T>);

impl<T: Ord> BinTree<T> {
    pub fn into_iter(self) -> InorderIntoIter<T> {
        InorderIntoIter(self)
    }
}

impl<T: Ord> Iterator for InorderIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct InorderIter<'tree, T: Ord> {
    curr_node: Option<&'tree TreeNode<T>>,
    node_stack: Vec<&'tree TreeNode<T>>,
}

impl<'tree, T: Ord> BinTree<T> {
    pub fn iter(&'tree self) -> InorderIter<'tree, T> {
        InorderIter {
            curr_node: self.root.0.as_deref(),
            node_stack: vec![],
        }
    }
}

impl<'tree, T: Ord> Iterator for InorderIter<'tree, T> {
    type Item = &'tree T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.curr_node {
            self.curr_node = node.left.0.as_deref();
            self.node_stack.push(node);
        }

        if let Some(popped_node) = self.node_stack.pop() {
            self.curr_node = popped_node.right.0.as_deref();
            return Some(&popped_node.item);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let tree: BinTree<i32> = BinTree::new();
        assert_eq!(tree, tree.clone());
    }

    #[test]
    fn inserting_values_in_order() {
        let mut tree: BinTree<i32> = BinTree::new();

        tree.insert(2);
        tree.insert(1);
        tree.insert(3);

        assert_eq!(tree.size(), 3);
        assert_eq!(tree, tree.clone());
    }

    #[test]
    fn iterating() {
        let mut tree: BinTree<u32> = BinTree::new();

        tree.insert(2);
        tree.insert(1);
        tree.insert(3);

        let mut tree_into_iter = tree.into_iter();

        assert_eq!(tree_into_iter.next(), Some(1));
        assert_eq!(tree_into_iter.next(), Some(2));
        assert_eq!(tree_into_iter.next(), Some(3));

        tree = BinTree::new();

        tree.insert(2);
        tree.insert(1);
        tree.insert(3);

        let mut tree_iter = tree.iter();

        assert_eq!(tree_iter.next(), Some(&1));
        assert_eq!(tree_iter.next(), Some(&2));
        assert_eq!(tree_iter.next(), Some(&3));
    }

    // #[test]
    // fn inorder_popping() {
    //     let mut tree: BinTree<u32> = BinTree::new();

    //     tree.insert(75);
    //     tree.insert(25);
    //     tree.insert(50);

    //     assert_eq!(tree.pop(), Some(25));
    //     assert_eq!(tree.pop(), Some(50));
    //     assert_eq!(tree.pop(), Some(75));
    // }
}
