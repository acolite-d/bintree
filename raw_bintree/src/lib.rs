use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};
use std::ptr;

// type Tree<T> = *mut TreeNode<T>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Tree<T: Ord>(*mut TreeNode<T>);

impl<T: Ord> Default for Tree<T> {
    #[inline]
    fn default() -> Self {
        Self(ptr::null_mut())
    }
}

// impl<T: Ord> Drop for Tree<T> {
//     fn drop(&mut self) {
//         if !self.0.is_null() {
//             let _ = unsafe {*self.0.drop_in_place()}
//         }
//     }
// }

#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode<T: Ord> {
    item: T,
    left: Tree<T>,
    right: Tree<T>,
    height: usize
}

impl<T: Ord> Into<Tree<T>> for TreeNode<T> {
    #[inline]
    fn into(self) -> Tree<T> {
        Tree(Box::into_raw(Box::new(self)))
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BinaryTree<T: Ord> {
    root: Tree<T>,
    size: usize,
}

impl<T: Ord> Default for BinaryTree<T> {
    fn default() -> Self {
        Self {
            root: Tree::default(),
            size: 0,
        }
    }
}

impl<T: Ord> TreeNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            left: Tree(ptr::null_mut()),
            right: Tree(ptr::null_mut()),
            height: 1
        }
    }
}

impl<T: Ord> Tree<T> {
    fn search(&self, target: &T) -> Option<&TreeNode<T>> {
        let mut tree_ptr = self.0;

        while let Some(node) = unsafe { tree_ptr.as_ref() } {
            match target.cmp(&node.item) {
                Ordering::Less => tree_ptr = node.left.0,
                Ordering::Greater => tree_ptr = node.right.0,
                Ordering::Equal => {
                    return Some(&node)
                }
            }
        }

        None
    }

    fn add_child(&mut self, new_item: T) {
        let mut tree = self;

        while let Some(node) = unsafe { tree.0.as_mut() } {

            match new_item.cmp(&node.item) {
                Ordering::Less => tree = &mut node.left,
                Ordering::Greater => tree = &mut node.right,
                Ordering::Equal => { return; }
            }
        }

        *tree = TreeNode::new(new_item).into();
    }

    fn remove_leftmost_child(&mut self) -> Option<TreeNode<T>> {

        let mut tree = self.0;

        unsafe {
            if let Some(node) = tree.as_mut()  {

                let pruned = loop {
                    match (node.left.0.as_mut(), node.right.0.as_mut()) {
                        (Some(_), None) => { tree = node.left.0},
                        (None, None) => {  }
                        _ => { }
                    }

                    2u32
                };


                None
            } else {
                None
            }
        }


        // if let None = unsafe { self.0.as_mut() } {
        //     return None;
        // }

        // let mut tree = self.0;



        // while let Some(node) = unsafe { tree.as_mut() } {
        //     if !node.left.0.is_null() {
        //         tree = node.left.0;
        //     } else if !node.right.0.is_null() {
        //         let ret_node = unsafe { node.left.0.replace(node.right.0.read()) };
        //     } else {

        //     }
        // }

        // None
    }
}

impl<T> BinaryTree<T>
where
    T: Ord,
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn remove_inorder(&mut self) -> Option<T> {
        None
    }
}

impl<T: Ord> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        while let Some(_) = self.remove_inorder() {}
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

impl<'tree, T: Ord> BinaryTree<T> {
    pub fn iter(&'tree self) -> InorderIter<'tree, T> {
        InorderIter {
            curr_node: unsafe { self.root.0.as_ref() },
            node_stack: vec![]
        }
    }
}

impl<'tree, T: Ord> Iterator for InorderIter<'tree, T> {
    type Item = &'tree T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.curr_node {
            self.curr_node = unsafe { node.left.0.as_ref() };
            self.node_stack.push(node);
        }

        if let Some(popped_node) = self.node_stack.pop() {
            self.curr_node = unsafe { popped_node.right.0.as_ref() };
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

// impl<T: Ord> FromIterator<T> for BinaryTree<T> {
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
//         let mut tree: BinaryTree<T> = Self::new();

//         iter.into_iter().for_each(|item| tree.insert(item));

//         tree
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_constructing() {
//         let default = BinaryTree::<i32>::default();
//         let new = BinaryTree::<i32>::new();

//         let vec = vec![-1, 0, 1, 2, 3];
//         // let collect: BinaryTree<i32> = BinaryTree::from_iter(vec);
//         // let cloned  = collect.clone();

//         assert_eq!(default, new);
//     }

//     // #[test]
//     // fn test_coercing() {
//     //     let mut tree = BinaryTree::<u8>::new();

//     //     tree.insert(1);
//     //     tree.insert(2);
//     //     tree.insert(3);

//     //     assert_eq!(tree.remove_inorder(), Some(1));
//     // }

//     #[test]
//     fn test_inserting() {
//         let mut tree: BinaryTree<u32> = BinaryTree::new();

//         tree.insert(3);
//         tree.insert(1);
//         tree.insert(2);

//         assert_eq!(tree.size(), 3);

//         let mut tree_iter = tree.iter();

//         assert_eq!(tree_iter.next(), Some(&1));
//         assert_eq!(tree_iter.next(), Some(&2));
//         assert_eq!(tree_iter.next(), Some(&3));

//         tree = BinaryTree::new();

//         tree.insert(5);
//         tree.insert(3);
//         tree.insert(1);
//         tree.insert(2);
//         tree.insert(4);

//         assert_eq!(tree.size(), 5);

//         tree_iter = tree.iter();

//         assert_eq!(tree_iter.next(), Some(&1));
//         assert_eq!(tree_iter.next(), Some(&2));
//         assert_eq!(tree_iter.next(), Some(&3));
//         assert_eq!(tree_iter.next(), Some(&4));
//         assert_eq!(tree_iter.next(), Some(&5));
//     }

//     #[test]
//     fn test_removing() {
//         let mut tree: BinaryTree<u64> = BinaryTree::new();

//         tree.insert(5);
//         tree.insert(3);
//         tree.insert(1);
//         tree.insert(2);
//         tree.insert(4);

//         for val in 1..=5 {
//             assert_eq!(tree.remove_inorder(), Some(val));
//         }

//         assert_eq!(tree.remove_inorder(), None);
//     }

//     #[test]
//     fn test_iterating() {
//         let mut tree: BinaryTree<char> = BinaryTree::new();

//         tree.insert('j');
//         tree.insert('a');
//         tree.insert('d');

//         // Non-consuming iterator
//         for (test_val, tree_val) in ['a', 'd', 'j'].iter().zip(tree.iter()) {
//             assert_eq!(test_val, tree_val)
//         }

//         // Consuming
//         for (test_val, tree_val) in ['a', 'd', 'j'].into_iter().zip(tree) {
//             assert_eq!(test_val, tree_val)
//         }
//     }
// }
