use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};
use std::mem;
use std::ptr;


#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode<T: Ord> {
    item: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            left: None,
            right: None,
        }
    }
}

#[inline]
fn get_mut_ref_to_leaf<'tree, T: Ord>(
    curr_node: &'tree mut TreeNode<T>,
    val: &T,
) -> &'tree mut Option<Box<TreeNode<T>>> {
    match val.cmp(&curr_node.item) {
        Ordering::Less => &mut curr_node.left,
        Ordering::Greater => &mut curr_node.right,
        Ordering::Equal => &mut curr_node.left,
    }
}

fn get_mut_ref_left<'tree, T: Ord>(
    curr_node: &'tree mut TreeNode<T>,
) -> &'tree mut Option<Box<TreeNode<T>>> {
    &mut curr_node.left
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct BinTree<T: Ord> {
    root: Option<Box<TreeNode<T>>>,
    size: usize,
}

impl<T> BinTree<T>
where
    T: Ord,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, new_item: T) {
        if let Some(root) = self.root.as_deref_mut() {
            let mut next_node = get_mut_ref_to_leaf(root, &new_item);

            while let Some(n) = next_node {
                next_node = get_mut_ref_to_leaf(n, &new_item);
            }

            next_node.replace(Box::new(TreeNode::new(new_item)));
        } else {
            self.root.replace(Box::new(TreeNode::new(new_item)));
        }

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(root) = self.root.as_deref_mut() {
            
            let mut next_node = get_mut_ref_left(root);

            let res = next_node.take().map(|n| n.item);
            if res.is_none() { println!("is none")}

            res

        } else {
            None
        }
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
            curr_node: self.root.as_deref(),
            node_stack: vec![],
        }
    }
}

impl<'tree, T: Ord> Iterator for InorderIter<'tree, T> {
    type Item = &'tree T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.curr_node {
            self.curr_node = node.left.as_deref();
            self.node_stack.push(node);
        }

        if let Some(popped_node) = self.node_stack.pop() {
            self.curr_node = popped_node.right.as_deref();
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
    fn inserting_values() {
        let mut tree: BinTree<i32> = BinTree::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        assert_eq!(tree, tree.clone());
    }

    #[test]
    fn iterating() {
        let mut tree: BinTree<u32> = BinTree::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        let mut tree_iter = tree.iter();

        assert_eq!(tree_iter.next(), Some(&1));
        assert_eq!(tree_iter.next(), Some(&2));
        assert_eq!(tree_iter.next(), Some(&3));

        tree = BinTree::new();

        tree.insert(5);
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        tree.insert(4);

        tree_iter = tree.iter();

        assert_eq!(tree_iter.next(), Some(&1));
        assert_eq!(tree_iter.next(), Some(&2));
        assert_eq!(tree_iter.next(), Some(&3));
        assert_eq!(tree_iter.next(), Some(&4));
        assert_eq!(tree_iter.next(), Some(&5));
    }

    #[test]
    fn inorder_popping() {
        let mut tree: BinTree<u32> = BinTree::new();

        tree.insert(75);
        tree.insert(25);
        tree.insert(50);

        assert_eq!(tree.pop(), Some(25));
        assert_eq!(tree.pop(), Some(50));
        assert_eq!(tree.pop(), Some(75));
    }
}
