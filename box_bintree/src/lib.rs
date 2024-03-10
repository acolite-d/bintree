use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::{FromIterator, IntoIterator};
use std::mem;

#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode<T: Ord + Debug> {
    item: T,
    left: Tree<T>,
    right: Tree<T>,
    height: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Tree<T: Ord + Debug>(Option<Box<TreeNode<T>>>);

impl<T: Ord + Debug> TreeNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            left: Tree(None),
            right: Tree(None),
            height: 1,
        }
    }
}

impl<T: Ord + Debug> From<TreeNode<T>> for Tree<T> {
    fn from(node: TreeNode<T>) -> Self {
        Tree(Some(Box::new(node)))
    }
}

impl<T: Ord + Debug> Default for Tree<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Ord + Debug> Tree<T> {


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

                self.calculate_height();
            }
        }

        match self.calculate_balance() {
            -2 => {
                println!(
                    "Right rotation, {:?} pivot",
                    self.0.as_deref().unwrap().item
                );

                let needs_left_rotate = self
                    .0
                    .as_deref()
                    .map(|n| n.left.0.as_deref())
                    .flatten()
                    .map(|n| n.left.0.as_deref())
                    .flatten()
                    .map(|n| n.right.0.as_deref())
                    .flatten()
                    .is_some();

                if needs_left_rotate {
                    println!("Needed extra rotate");
                    self.0.as_deref_mut().map(|n| {
                        n.right.rotate_left();
                        n
                    });
                }

                self.rotate_right();
                self.calculate_height();
            }

            2 => {
                println!("Left rotation, {:?} pivot", self.0.as_deref().unwrap().item);

                let needs_right_rotate = self
                    .0
                    .as_deref()
                    .map(|n| n.right.0.as_deref())
                    .flatten()
                    .map(|n| n.right.0.as_deref())
                    .flatten()
                    .map(|n| n.left.0.as_deref())
                    .flatten()
                    .is_some();

                if needs_right_rotate {
                    println!("Needed extra rotate");
                    self.0.as_deref_mut().map(|n| {
                        n.right.rotate_right();
                        n
                    });
                }

                self.rotate_left();
                self.calculate_height();
            }

            (-1..=1) => {}

            _ => unreachable!(),
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

    /*
     *          P
     *        /   \
     *       /     \
     *      T1      X
     *             / \
     *            /   \
     *           Y    New
     *
     *
     *          X
     *        /   \
     *       /     \
     *      P      New
     *     / \
     *    T1  Y
     */
    fn rotate_left(&mut self) {
        let mut pivot = self.0.take();
        let mut x: Tree<T> = Tree::default();
        let mut y: Tree<T> = Tree::default();

        pivot.as_deref_mut().map(|p| {
            let mut right_child = p.right.0.take();

            let left_of_right = right_child
                .as_deref_mut()
                .map(|r| r.left.0.take())
                .flatten()
                .take();

            x = Tree(right_child);
            y = Tree(left_of_right);

            p.right = y;
            *self = x;
            // p.height -= 2;

            p
        });

        self.0.as_deref_mut().map(|x| {
            x.left = Tree(pivot);
            x
        });
    }

    fn rotate_right(&mut self) {
        let mut pivot = self.0.take();
        let mut x: Tree<T> = Tree::default();
        let mut y: Tree<T> = Tree::default();

        pivot.as_deref_mut().map(|p| {
            let mut left_child = p.left.0.take();

            let right_of_left = left_child
                .as_deref_mut()
                .map(|l| l.right.0.take())
                .flatten()
                .take();

            x = Tree(left_child);
            y = Tree(right_of_left);

            p.left = y;
            *self = x;

            p
        });

        self.0.as_deref_mut().map(|x| {
            x.right = Tree(pivot);
            x
        });
    }

    fn calculate_balance(&self) -> i8 {
        if let Some(TreeNode { left, right, .. }) = self.0.as_deref() {
            right.0.as_deref().map_or(0, |n| (n.height) as i8)
                - left.0.as_deref().map_or(0, |n| (n.height) as i8)
        } else {
            0
        }
    }

    #[allow(unused)]
    fn calculate_height(&mut self) -> usize {
        let height = self.0.as_deref_mut().map_or(0, |node| {
            let l_subtree_height = 1 + node.left.calculate_height();

            let r_subtree_height = 1 + node.right.calculate_height();

            match l_subtree_height.cmp(&r_subtree_height) {
                Ordering::Less => {
                    // println!("Changing the node ({:?}) with value {:?} to have height of {}", node as *const TreeNode<T>, &node.item, r_subtree_height);
                    node.height = r_subtree_height;
                    r_subtree_height
                }

                Ordering::Greater | Ordering::Equal => {
                    // println!("Changing the node ({:?}) with value {:?} to have height of {}", node as *const TreeNode<T>, &node.item, r_subtree_height);
                    node.height = l_subtree_height;
                    l_subtree_height
                }
            }
        });

        height
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct BinTree<T: Ord + Debug> {
    root: Tree<T>,
    size: usize,
}

impl<T: Ord + Debug> BinTree<T> {
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

    #[inline]
    pub fn insert(&mut self, new_item: T) {
        self.root.add_child(new_item);
        self.size += 1;
    }

    #[inline]
    pub fn remove_inorder(&mut self) -> Option<T> {
        let removed = self.root.remove_leftmost_child().map(|n| {
            self.size -= 1;
            n.item
        });

        removed
    }
}

pub struct InorderIntoIter<T: Ord + Debug>(BinTree<T>);

impl<T: Ord + Debug> Iterator for InorderIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove_inorder()
    }
}

pub struct InorderIter<'tree, T: Ord + Debug> {
    curr_node: Option<&'tree TreeNode<T>>,
    node_stack: Vec<&'tree TreeNode<T>>,
}

impl<'tree, T: Ord + Debug> BinTree<T> {
    pub fn iter(&'tree self) -> InorderIter<'tree, T> {
        InorderIter {
            curr_node: self.root.0.as_deref(),
            node_stack: vec![],
        }
    }
}

impl<'tree, T: Ord + Debug> Iterator for InorderIter<'tree, T> {
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

impl<T: Ord + Debug> IntoIterator for BinTree<T> {
    type Item = T;
    type IntoIter = InorderIntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        InorderIntoIter(self)
    }
}

impl<T: Ord + Debug> FromIterator<T> for BinTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree: BinTree<T> = Self::new();
        iter.into_iter().for_each(|item| tree.insert(item));
        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<T: Ord + Debug> BinTree<T> {
        fn from_raw_tree_unchecked(tree: Tree<T>) -> Self {
            BinTree {
                root: tree,
                size: 0,
            }
        }
    }

    impl<T: Ord + Debug> Tree<T> {
        fn add_child_unbalanced(&mut self, new_item: T) {
            match self.0.as_deref_mut() {
                None => *self = TreeNode::new(new_item).into(),

                Some(node) => {
                    match new_item.cmp(&node.item) {
                        Ordering::Less => node.left.add_child(new_item),
                        Ordering::Greater => node.right.add_child(new_item),
                        Ordering::Equal => {}
                    };

                    node.height += 1;
                }
            }
        }

        #[allow(unused)]
        fn add_subtree_left(&mut self, mut subtree: Self) {
            let subtree_height = subtree.calculate_height();
    
            self.0.as_deref_mut().map(|n| {
                n.left = subtree;
    
                if n.height < subtree_height + 1 {
                    n.height = subtree_height + 1;
                }
            });
        }
    
        #[allow(unused)]
        fn add_subtree_right(&mut self, mut subtree: Self) {
            let subtree_height = subtree.calculate_height();
    
            self.0.as_deref_mut().map(|n| {
                n.right = subtree;
    
                if n.height < subtree_height + 1 {
                    n.height = subtree_height + 1;
                }
            });
        }
    }

    macro_rules! tree {

        ($parent:expr, L {$($l_tt:tt)*}, R {$($r_tt:tt)*}) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$($l_tt)*});
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

        ($parent:expr, L $l_child:expr, R {$($r_tt:tt)*}) => {
            {
                let mut tree: Tree<_> = TreeNode::new($parent).into();
                tree.add_subtree_left(tree!{$l_child});
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

        () => {
            Tree(None)
        };

        ($val:expr) => {
            TreeNode::new($val).into()
        };
    }

    // #[test]
    // fn build_tree_macro() {
    //     assert_eq!(tree! {}, Tree::<u8>(None));

    //     let just_root: Tree<u8> = tree! {1_u8};
    //     assert_eq!(just_root, TreeNode::new(1_u8).into());

    //     let null_on_left: Tree<u8> = tree! {
    //         2, L {}, R 3
    //     };
    //     let mut manual_tree: Tree<u8> = TreeNode::new(2).into();
    //     manual_tree.add_child_unbalanced(3);

    //     assert_eq!(null_on_left, manual_tree);

    //     let null_on_right: Tree<u8> = tree! {
    //         2, L 1, R {}
    //     };
    //     manual_tree = TreeNode::new(2).into();
    //     manual_tree.add_child_unbalanced(1);

    //     assert_eq!(null_on_right, manual_tree);

    //     let children_are_subtrees: Tree<u8> = tree! {
    //         5,
    //             L {
    //                 3, L 2, R 4
    //             },
    //             R {
    //                 7, L 6, R 8
    //             }
    //     };

    //     manual_tree = TreeNode::new(5).into();

    //     [3, 2, 4, 7, 6, 8]
    //         .into_iter()
    //         .for_each(|val| manual_tree.add_child_unbalanced(val));

    //     assert_eq!(children_are_subtrees, manual_tree);

    //     let skew_left = tree! {
    //         5, L {
    //             4, L {
    //                 3, L {
    //                     2, L {
    //                         1, L {}, R {}
    //                     }, R {}
    //                 }, R {}
    //             }, R {}
    //         }, R {}
    //     };

    //     manual_tree = Tree(None);
    //     [5, 4, 3, 2, 1]
    //         .into_iter()
    //         .for_each(|val| manual_tree.add_child_unbalanced(val));

    //     assert_eq!(skew_left, manual_tree);

    //     let skew_right = tree! {
    //         1, L {}, R {
    //             2, L {}, R {
    //                 3, L {}, R {
    //                     4, L {}, R {
    //                         5, L {}, R {}
    //                     }
    //                 }
    //             }
    //         }
    //     };

    //     manual_tree = Tree(None);
    //     [1, 2, 3, 4, 5]
    //         .into_iter()
    //         .for_each(|val| manual_tree.add_child_unbalanced(val));

    //     assert_eq!(skew_right, manual_tree);
    // }

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
    fn inserting_values_with_established_balanced() {
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
    fn inorder_removal_with_established_balance() {
        let mut tree: BinTree<u32> = BinTree::new();

        tree.insert(50);
        tree.insert(25);
        tree.insert(75);

        assert_eq!(tree.remove_inorder(), Some(25));
        assert_eq!(tree.remove_inorder(), Some(50));
        assert_eq!(tree.remove_inorder(), Some(75));
    }

    #[test]
    fn clearing() {
        let mut bintree = BinTree::from_raw_tree_unchecked(tree! {
            'B', L 'A', R 'C'
        });

        bintree.clear();

        assert_eq!(bintree, BinTree::new());
        assert_eq!(bintree.size, 0);
    }

    #[test]
    fn searching() {
        let tree: Tree<u32> = tree! {
            50,
                L {
                    25,
                        L {
                            10, L 5, R 15
                        },
                        R {
                            40, L 35, R 45
                        }
                },
                R {
                    75,
                        L {
                            60, L 65, R 70
                        },
                        R {
                            90, L 95, R 85
                        }
                }
        };

        let bintree = BinTree::from_raw_tree_unchecked(tree);

        assert_eq!(bintree.search(&75), Some(&75));
        assert_eq!(bintree.search(&10), Some(&10));
        assert_eq!(bintree.search(&60), Some(&60));
        assert_eq!(bintree.search(&35), Some(&35));
        assert_eq!(bintree.search(&100), None);
    }

    #[test]
    fn rotating() {
        // Right rotation
        let mut unbalanced_left: Tree<u32> = tree! {
            3, L {2, L 1, R {} }, R {}
        };

        unbalanced_left.rotate_right();
        unbalanced_left.calculate_height();

        let expected_result = tree! {
            2, L 1, R 3
        };

        assert_eq!(unbalanced_left, expected_result);

        //Left Rotation
        let mut unbalanced_right = tree! {
            1, L {}, R {2, L {}, R 3}
        };

        unbalanced_right.rotate_left();
        unbalanced_right.calculate_height();

        assert_eq!(
            unbalanced_right,
            tree! {
                2, L 1, R 3
            }
        );

        //Right-Left Rotation
        let mut zig_right_zag_left = tree! {
            3, L {}, R {5, L 4, R {}}
        };

        if let Some(TreeNode { right, .. }) = zig_right_zag_left.0.as_deref_mut() {
            right.rotate_right();
        }

        zig_right_zag_left.rotate_left();
        zig_right_zag_left.calculate_height();

        assert_eq!(
            zig_right_zag_left,
            tree! {
                4, L 3, R 5
            }
        );

        // Left-Right Rotation
        let mut zig_left_zag_right = tree! {
            5, L { 3, L {}, R 4 }, R {}
        };

        if let Some(TreeNode { left, .. }) = zig_left_zag_right.0.as_deref_mut() {
            left.rotate_left();
        }

        zig_left_zag_right.rotate_right();
        zig_left_zag_right.calculate_height();

        assert_eq!(
            zig_left_zag_right,
            tree! {
                4, L 3, R 5
            }
        );
    }

    #[test]
    fn maintaining_balance() {
        let mut balancing_tree: Tree<u32>;

        // Maintaining balance with a single left rotation?
        balancing_tree = tree! {};

        balancing_tree.add_child(30);
        balancing_tree.add_child(40);
        balancing_tree.add_child(50);

        assert_eq!(
            balancing_tree,
            tree! {
                40, L 30, R 50
            }
        );

        // // Maintaining balance with a single right rotation?
        balancing_tree = tree! {};
        balancing_tree.add_child(30);
        balancing_tree.add_child(20);
        balancing_tree.add_child(10);

        assert_eq!(
            balancing_tree,
            tree! {
                20, L 10, R 30
            }
        );

        // References pulled from illustrations on
        // Geeks4Geeks
        // https://www.geeksforgeeks.org/insertion-in-an-avl-tree/
        balancing_tree = tree! {
            13,
            L {
                10, L{5, L 4, R 6}, R 11
            },
            R {
                15, L {}, R 16
            }
        };

        balancing_tree.add_child(14);

        assert_eq!(
            balancing_tree,
            tree! {
                13,
                L {
                    10,
                    L {5, L 4, R 6},
                    R 11
                },
                R {
                    15, L 14, R 16
                }
            }
        );

        balancing_tree = tree! {
            13,
            L {
                10, L{5, L 4, R 8}, R 11
            },
            R {
                15, L {}, R 16
            }
        };

        balancing_tree.add_child(3);

        assert_eq!(
            balancing_tree,
            tree! {
                13,
                L {
                    5, L {4, L 3, R {}}, R {10, L 8, R 11}
                },
                R {
                    15, L {}, R 16
                }
            }
        );

        balancing_tree = tree! {
            30, L 5, R {35, L 32, R 40}
        };

        balancing_tree.add_child(45);

        assert_eq!(
            balancing_tree,
            tree! {
                35,
                L {30, L 5, R 32},
                R {40, L {}, R 45}
            }
        );

        // balancing_tree = tree! {
        //     13,
        //     L {
        //         10, L{5, L 4, R 6}, R 11
        //     },
        //     R {
        //         15, L {}, R 16
        //     }
        // };

        // println!("added 7");

        // balancing_tree.add_child(7);

        // assert_eq!(
        //     balancing_tree,
        //     tree! {
        //         13,
        //         L {
        //             6, L {5, L 4, R {}}, R {10, L 7, R 11}
        //         },
        //         R {
        //             15, L {}, R 16
        //         }
        //     }
        // );

        balancing_tree = tree! {
            5,
            L {
                2, L 1, R{4, L 3, R {}}
            },
            R {
                7, L 6, R {9, L {}, R 16}
            }
        };

        balancing_tree.add_child(15);

        assert_eq!(
            balancing_tree,
            tree! {
                5,
                L {
                    2, L 1, R {4, L 3, R {}}
                },
                R {
                    7, L 6, R {15, L 9, R 16}
                }
            }
        )
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
