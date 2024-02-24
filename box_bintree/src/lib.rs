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

impl<T: Ord> BinTree<T>
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

    macro_rules! tree {

        ($parent:expr, L {$($l_tt:tt)*}, R {$($r_tt:tt)*}) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$($l_tt)*});
                tree.add_subtree_right(tree!{$($r_tt)*});
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

        ($parent:expr, L {$($l_tt:tt)*}, R $r_child:expr) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$($l_tt)*});
                tree.add_subtree_right(tree!{$r_child});
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
        
        () => {
            Tree(None) 
        };

        ($val:expr) => {
            TreeNode::new($val).into()
        };
    }

    #[test]
    fn build_tree_macro() {

        assert_eq!(tree!{}, Tree::<u8>(None));

        let just_root: Tree<u8> = tree!{1_u8};
        assert_eq!(just_root, TreeNode::new(1_u8).into());

        let null_on_left: Tree<u8> = tree!{ 
            2, L {}, R 3 
        };
        let mut manual_tree: Tree<u8> = TreeNode::new(2).into();
        manual_tree.add_child(3);

        assert_eq!(null_on_left, manual_tree);

        let null_on_right: Tree<u8> = tree!{
            2, L 1, R {}
        };
        manual_tree = TreeNode::new(2).into();
        manual_tree.add_child(1);

        assert_eq!(null_on_right, manual_tree);

        let children_are_subtrees: Tree<u8> = tree!{
            5, 
                L {
                    3, L 2, R 4
                }, 
                R {
                    7, L 6, R 8
                }
        };

        manual_tree = TreeNode::new(5).into();

        [3, 2, 4, 7, 6, 8]
            .into_iter()
            .for_each(|val| manual_tree.add_child(val));

        assert_eq!(children_are_subtrees, manual_tree);

        let skew_left = tree!{
            5, L {
                4, L {
                    3, L {
                        2, L {
                            1, L {}, R {}
                        }, R {}
                    }, R {}
                }, R {}
            }, R {}
        };


        manual_tree = Tree(None);
        [5, 4, 3, 2, 1]
            .into_iter()
            .for_each(|val| manual_tree.add_child(val));

        assert_eq!(skew_left, manual_tree);

        let skew_right = tree!{
            1, L {}, R {
                2, L {}, R {
                    3, L {}, R {
                        4, L {}, R {
                            5, L {}, R {}
                        }
                    }
                }
            }
        };

        manual_tree = Tree(None);
        [1, 2, 3, 4, 5]
            .into_iter()
            .for_each(|val| manual_tree.add_child(val));

        assert_eq!(skew_right, manual_tree);
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

    #[test]
    fn inorder_popping_with_order() {
        let mut tree: BinTree<u32> = BinTree::new();

        tree.insert(50);
        tree.insert(25);
        tree.insert(75);

        assert_eq!(tree.pop(), Some(25));
        assert_eq!(tree.pop(), Some(50));
        assert_eq!(tree.pop(), Some(75));
    }

    #[test]
    fn searching_with_order() {
        let tree: BinTree<u8> = vec![50, 25, 75, 90, 60, 20, 5].into_iter().collect();

        assert_eq!(tree.search(&50), Some(&50));
        assert_eq!(tree.search(&5), Some(&5));
        assert_eq!(tree.search(&90), Some(&90));
        assert_eq!(tree.search(&60), Some(&60));
        assert_eq!(tree.search(&20), Some(&20));

        assert_eq!(tree.search(&111), None);
    }

    #[test]
    fn rotating() {

        // Right rotation
        let mut unbalanced_left: Tree<u32> = tree!{
            3, L {2, L 1, R {} }, R {} 
        };

        unbalanced_left.rotate_right();

        assert_eq!(
            unbalanced_left,
            tree!{
                2, L 1, R 3
            }
        );

        //Left Rotation
        let mut unbalanced_right = tree!{
            1, L {}, R {2, L {}, R 3}
        };

        unbalanced_right.rotate_left();

        assert_eq!(
            unbalanced_right,
            tree!{
                2, L 1, R 3
            }
        );
    }

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
