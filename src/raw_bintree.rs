use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};
use std::ptr;

macro_rules! alloc_node {
    ($item:expr) => {
        Box::into_raw(Box::new(TreeNode::new($item)))
    };
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode<T: Ord>
{
    item: T,
    left: *mut TreeNode<T>,
    right: *mut TreeNode<T>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BinaryTree<T: Ord>
{
    root: *mut TreeNode<T>,
    size: usize,
}

impl<T: Ord> Default for BinaryTree<T> {
    fn default() -> Self {
        Self {
            root: ptr::null_mut(),
            size: 0,
        }
    }
}

impl<T: Ord> TreeNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }
}

impl<T> BinaryTree<T>
where
    T: Ord,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            root: ptr::null_mut(),
            size: 0,
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, new_item: T) {

        // Iterate down the tree, find the leaf has the NULL child
        // to replace with the new target item.

        if self.root.is_null() {
            self.root = alloc_node!(new_item);
        } else {

            let mut next_node = self.root;
            let mut curr_node = ptr::null_mut::<TreeNode<T>>();
    
            while !next_node.is_null() {
                curr_node = next_node;
                let curr_val = unsafe { &(*curr_node).item };
    
                next_node = match new_item.cmp(curr_val) {
                    Ordering::Greater => unsafe { (*curr_node).right },
                    Ordering::Less | Ordering::Equal => unsafe { (*curr_node).left },
                };
            }
    
            let curr_val = unsafe { &(*curr_node).item };
    
            match new_item.cmp(curr_val) {
                Ordering::Greater => unsafe { (*curr_node).right = alloc_node!(new_item) }
                Ordering::Less | Ordering::Equal => unsafe { (*curr_node).left = alloc_node!(new_item) }
            }
        }

        self.size += 1;
    }

    pub fn remove_from_root(&mut self) -> Option<T> {
        todo!()
    }

    pub fn remove_inorder(&mut self) -> Option<T> {
        if !self.root.is_null() {
            // What is the logic here?
            // Need to take the leftmost tree node
            // set the parent of that node, parent.left == null
            // call destructors to free nodes

            let mut curr_node = self.root;
            let mut parent_node = self.root;

            unsafe {
                while !(*curr_node).left.is_null() {
                    parent_node = curr_node;
                    curr_node = (*curr_node).left;
                }

                let item = ptr::read(curr_node as *const TreeNode<T>).item;

                if !(*curr_node).right.is_null() {
                    let right_child = (*curr_node).right;
                    (*parent_node).left = right_child;
                } else if curr_node != parent_node {
                    (*parent_node).left = ptr::null_mut();
                } else {
                    self.root = ptr::null_mut();
                }

                ptr::drop_in_place(curr_node);
                
                Some(item)
            }
        } else {
            None
        }
    }

    pub fn into_iter(self) -> InorderIntoIter<T> {
        InorderIntoIter(self)
    }

    pub fn iter(&self) -> InorderIter<'_, T> {
        InorderIter {
            curr_node: match self.root.is_null() {
                true => None,
                false => unsafe { self.root.as_ref() },
            },
            node_stack: vec![]
        }
    }
}

impl<T: Ord> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        while let Some(_) = self.remove_inorder() { }
    }
}

pub struct InorderIntoIter<T: Ord>(BinaryTree<T>);

impl<T: Ord> Iterator for InorderIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove_inorder()
    }
}

pub struct InorderIter<'tree, T: Ord> {
    curr_node: Option<&'tree TreeNode<T>>,
    node_stack: Vec<&'tree TreeNode<T>>,
}

impl<'tree, T: Ord> Iterator for InorderIter<'tree, T> {
    type Item = &'tree T;

    fn next(&mut self) -> Option<Self::Item> {

        while let Some(node) = self.curr_node {
            self.curr_node = unsafe { node.left.as_ref() };
            self.node_stack.push(node);
        }

        if let Some(popped_node) = self.node_stack.pop() {
            self.curr_node = unsafe { popped_node.right.as_ref() };
            return Some(&popped_node.item);
        }

        None
    }
}

impl<T: Ord> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = InorderIntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        InorderIntoIter(self)
    }
}

impl<T: Ord> FromIterator<T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree: BinaryTree<T> = Self::new();

        for val in iter {
            tree.insert(val);
        }

        tree
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constructing() {
        let default = BinaryTree::<i32>::default();
        let new     = BinaryTree::<i32>::new();

        let vec     = vec!(-1,0,1,2,3);
        let collect: BinaryTree<i32> = BinaryTree::from_iter(vec);
        // let cloned  = collect.clone();

        assert_eq!(default, new);
    }

    #[test]
    fn test_inserting() {
        let mut tree: BinaryTree<u32> = BinaryTree::new();

        tree.insert(3);
        tree.insert(1);
        tree.insert(2);

        assert_eq!(tree.size(), 3);

        let mut tree_iter = tree.iter();

        assert_eq!(tree_iter.next(), Some(&1));
        assert_eq!(tree_iter.next(), Some(&2));
        assert_eq!(tree_iter.next(), Some(&3));

        tree = BinaryTree::new();

        tree.insert(5);
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        tree.insert(4);

        assert_eq!(tree.size(), 5);

        tree_iter = tree.iter();

        assert_eq!(tree_iter.next(), Some(&1));
        assert_eq!(tree_iter.next(), Some(&2));
        assert_eq!(tree_iter.next(), Some(&3));
        assert_eq!(tree_iter.next(), Some(&4));
        assert_eq!(tree_iter.next(), Some(&5));
    }

    #[test]
    fn test_removing() {
        let mut tree: BinaryTree<u64> = BinaryTree::new();

        tree.insert(5);
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        tree.insert(4);

        for val in 1..=5 {
            assert_eq!(tree.remove_inorder(), Some(val));
        }

        assert_eq!(tree.remove_inorder(), None);
    }

    #[test]
    fn test_iterating() {
        let mut tree: BinaryTree<char> = BinaryTree::new();

        tree.insert('j');
        tree.insert('a');
        tree.insert('d');


        // non-consuming iterator
        for (test_val, tree_val) in ['a', 'd', 'j'].iter().zip(tree.iter()) {
            assert_eq!(test_val, tree_val)
        }

        // Consuming 
        for (test_val, tree_val) in ['a', 'd', 'j'].into_iter().zip(tree) {
            assert_eq!(test_val, tree_val)
        }
    }
}