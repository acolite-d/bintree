use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};
use std::mem;


#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode<T: Ord> {
    item: T,
    left: Tree<T>,
    right: Tree<T>,
    // height: u64,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Tree<T: Ord>(Option<Box<TreeNode<T>>>);

impl<T: Ord> TreeNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            left: Tree(None),
            right: Tree(None),
            // height: 0
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
    fn add_subtree_left(&mut self, subtree: Self) {
        self.0.as_deref_mut().map(|n| {
            n.left = subtree;
        });
    }
    
    fn add_subtree_right(&mut self, subtree: Self) {
        self.0.as_deref_mut().map(|n| {
            n.right = subtree;
        });
    }

    fn search(&self, target: &T) -> Option<&TreeNode<T>> {
        match self.0.as_deref() {
            None => None,

            Some(node @ TreeNode { item, .. }) => match target.cmp(item) {
                Ordering::Less => node.left.search(target),
                Ordering::Greater => node.right.search(target),
                Ordering::Equal => Some(node),
            },
        }
    }

    fn add_child(&mut self, new_item: T) {
        match self.0.as_deref_mut() {
            None => *self = TreeNode::new(new_item).into(),

            Some(node) => {
                match new_item.cmp(&node.item) {
                    Ordering::Less => node.left.add_child(new_item),
                    Ordering::Greater => node.right.add_child(new_item),
                    Ordering::Equal => {}
                };

                // node.height += 1;
            }
        }
    }

    fn remove_leftmost_child(&mut self) -> Option<Box<TreeNode<T>>> {
        let pruned = match self.0.as_deref_mut() {
            None => None,

            Some(TreeNode {
                left: l @ Tree(Some(_)),
                ..
            }) => l.remove_leftmost_child(),

            Some(TreeNode {
                left: Tree(None),
                right: Tree(None),
                ..
            }) => self.0.take(),

            Some(TreeNode {
                left: Tree(None),
                right: r @ Tree(Some(_)),
                ..
            }) => {
                let right_child = r.0.take();
                mem::replace(&mut self.0, right_child)
            }
        };

        pruned
    }


    // Given pivot point of self, replace self with right, self becomes right's left
    // Should I take the pivot point here?
    /*
     *          Y
     *        /   \
     *       /     \
     *      T1      X       
     *             / \
     *            /   \
     *           T2    Z 
     * 
     * 
     *          X
     *        /   \
     *       /     \
     *      Y       Z 
     *     / \     
     *    T1 T2   
     *           
     *                 
     * LEFT PIVOT ON Y
     * 1. Take Y subtree
     * 2. Take child X, take T2 as well
     * 3. Put T2 on Y right side
     * 4. Put Y on left side of X
     * 5. Put the resulting subtree back on self
     */

    #[allow(unused)]
    fn rotate_left(&mut self) {
        let mut y = self.0.take();
        let x: Tree<T>;

        if let Some(TreeNode { right, ..}) = y.as_deref_mut() {
            x = Tree(right.0.take());
            *self = x;
        }

        self.0.as_deref_mut().map(|pivot| { 
            pivot.left = Tree(y); 
            pivot
        });
    }

    #[allow(unused)]
    fn rotate_right(&mut self) {
        let mut y = self.0.take();
        let x: Tree<T>;

        if let Some(TreeNode { left, .. }) = y.as_deref_mut() {
            x = Tree(left.0.take());
            *self = x;
        }

        self.0.as_deref_mut().map(|pivot| { 
            pivot.right = Tree(y); 
            pivot
        });
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
    pub fn search(&self, target: &T) -> Option<&T> {
        self.root.search(target).map(|node| &node.item)
    }


    #[inline]
    pub fn insert(&mut self, new_item: T) {
        self.root.add_child(new_item);
        self.size += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let popped = self.root.remove_leftmost_child().map(|n| {
            self.size -= 1;
            n.item
        });

        popped
    }
}

pub struct InorderIntoIter<T: Ord>(BinTree<T>);

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

impl<T: Ord> IntoIterator for BinTree<T> {
    type Item = T;
    type IntoIter = InorderIntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        InorderIntoIter(self)
    }
}

impl<T: Ord> FromIterator<T> for BinTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree: BinTree<T> = Self::new();
        iter.into_iter().for_each(|item| tree.insert(item));
        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // build_tree!()
    // Recursive macro, but what do I need to write here
    /*
     * Empty Case: build_tree!() -> Tree(None)
     * Explicit Null case: build_tree!(Nil) -> Tree(None)
     * Single node case: build_tree!(1) -> Tree(Some(TreeNode(1)))
     * Recursive case: build_tree!(1 (2 3)) -> t = Tree(Some(TreeNode1)); l, r = build_tree!((2 3))
     */
    macro_rules! build_tree {
        ($val:expr, (($($left_tt:tt)*), ($($right_tt:tt)*))) => {
            {
                let mut tree: Tree<_> = TreeNode::new($val).into();

                tree.add_subtree_left(build_tree!($($left_tt)*));
                tree.add_subtree_right(build_tree!($($right_tt)*));
                tree
            }
        };
        
        // ($val:expr, ($($left_tt:tt)*, T$right_leaf:expr)) => {
        //     {
        //         let mut tree: Tree<_> = TreeNode::new($val).into();

        //         tree.add_subtree_left(build_tree!($($left_tt)*));
        //         tree.add_subtree_right(build_tree!($right_leaf));
        //         tree
        //     }
        // };

        // ($val:expr, ($left_leaf:expr, $($right_tt:tt)*)) => {
        //     {
        //         let mut tree: Tree<_> = TreeNode::new($val).into();

        //         tree.add_subtree_left(build_tree!($left_leaf));
        //         tree.add_subtree_right(build_tree!($($right_tt)*));
        //         tree
        //     }
        // };

        // ($val:expr, ($left_leaf:expr, $right_leaf:expr)) => {
        //     {
        //         let mut tree: Tree<_> = TreeNode::new($val).into();

        //         tree.add_subtree_left(build_tree!($left_leaf));
        //         tree.add_subtree_right(build_tree!($right_leaf));
        //         tree
        //     }
        // };

        () => {
            Tree(None)
        };

        (()) => {
            Tree(None)
        };

        ($val:expr) => {
            TreeNode::new($val).into()
        };
    }

    macro_rules! tree {

        () => {
            Tree(None) 
        };

        (n) => {
            Tree(None) 
        };

        ($val:expr) => {
            TreeNode::new($val).into()
        };

        ($parent:expr, L {$($l_tt:tt)*}, R {$($r_tt:tt)*}) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$($l_tt)*});
                tree.add_subtree_right(tree!{$($r_tt)*});
                tree
            }
        };

        ($parent:expr, L $l_child:expr, R $r_child:expr) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$l_child});
                tree.add_subtree_right(tree!{$r_child});
                tree
            }
        };

        ($parent:expr, L $l_child:expr, R {$($r_tt:tt)*}) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$l_child});
                tree.add_subtree_right(tree!{$($r_tt)*});
                tree
            }
        };

        ($parent:expr, L {$($l_tt:tt)*}, R N) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$($l_tt)*});
                tree.add_subtree_right(Tree(None));
                tree
            }
        };

        ($parent:expr, L {$($l_tt:tt)*}, R $r_child:expr) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$($l_tt)*});
                tree.add_subtree_right(tree!{$r_child});
                tree
            }
        };
    }

    #[test]
    fn build_tree_macro() {
        let empty_tree: Tree<u8> = tree!();
        assert_eq!(empty_tree, Tree(None));

        let just_root: Tree<u8> = tree!(1u8);
        assert_eq!(just_root, TreeNode::new(1u8).into());

        let simple: Tree<u8> = tree!{ 2, L 1, R N };
        let mut manual_tree: Tree<u8> = TreeNode::new(2).into();
        manual_tree.add_child(2);
        manual_tree.add_child(1);

        assert_eq!(simple, manual_tree);


        // let macro_tree: Tree<u8> = tree!{
        //     2, L 1, R { 4, L 3 R () }
        //     // 2, ((1), (4, ((3), ())))
        // };
        // let mut manual_tree: Tree<u8> = TreeNode::new(2).into();
        // manual_tree.add_child(4);
        // manual_tree.add_child(1);
        // manual_tree.add_child(3);

        // assert_eq!(macro_tree, manual_tree);
    }

    #[test]
    fn constructor() {
        let tree: BinTree<i32> = BinTree::new();
        assert_eq!(tree, tree.clone());

        let tree2: BinTree<char> = BinTree::default();
        assert_eq!(tree2, tree2.clone());

        let iter = vec![20, 10, 30].into_iter();
        let tree3: BinTree<u16> = iter.collect();
        assert_eq!(tree3, tree3.clone());

        for (tree_val, num) in tree3.into_iter().zip([10, 20, 30]) {
            assert_eq!(tree_val, num);
        }
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
    // fn inorder_popping_with_order() {
    //     let mut tree: BinTree<u32> = BinTree::new();

    //     tree.insert(50);
    //     tree.insert(25);
    //     tree.insert(75);

    //     assert_eq!(tree.pop(), Some(25));
    //     assert_eq!(tree.pop(), Some(50));
    //     assert_eq!(tree.pop(), Some(75));
    // }

    // #[test]
    // fn searching_with_order() {
    //     let tree: BinTree<u8> = vec![50, 25, 75, 90, 60, 20, 5].into_iter().collect();

    //     assert_eq!(tree.search(&50), Some(&50));
    //     assert_eq!(tree.search(&5), Some(&5));
    //     assert_eq!(tree.search(&90), Some(&90));
    //     assert_eq!(tree.search(&60), Some(&60));
    //     assert_eq!(tree.search(&20), Some(&20));

    //     assert_eq!(tree.search(&111), None);
    // }

    // #[test]
    // fn tree_rotating() {

    //     // Right rotation
    //     let mut tree1: Tree<u32> = build_tree!{
    //         3, ( (2, (1), ()) ), ()
    //     };

    //     tree1.add_child(2);
    //     tree1.add_child(1);

    //     tree1.rotate_right();

    //     let mut tree2 = Tree(
    //         Some(TreeNode::new(2u32).into())
    //     );

    //     tree2.add_child(1);
    //     tree2.add_child(3);

    //     assert_eq!(tree1, tree2);

    //     //Left Rotation
    //     tree1 = build_tree!{
    //         1, ( (), (2, ((), (3))))
    //     };
    //     tree1.rotate_left();

    //     tree2 = build_tree!(2, ( (1), (3)));
    //     assert_eq!(tree1, tree2);

    //     //LR Rotation
    // }

    #[test]
    fn send_sync() {
        fn is_send<T: Send>() {}
        fn is_sync<T: Sync>() {}

        is_send::<BinTree<i32>>();
        is_sync::<BinTree<i32>>();

        is_send::<BinTree<String>>();
        is_sync::<BinTree<String>>();
    }
}
