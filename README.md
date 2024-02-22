# Exploring Data Structures and Pointers in Rust

```Rust
/* RBL NOTES
 * The properties of an RBL tree follow a few key rules:
 * 1. Every node is either red or black
 * 2. The root is always black
 * 3. Every NULL node is considered black
 * 4. If a node is red, both children are black (no path can contain
 *    continguous red nodes)
 * 5. Every path from root to a NULL has same number of black nodes
 */

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Color {
    Red,
    Black
}

impl Color {
    fn swap(&mut self) {
        
    }
}
```

## RBL Approach
INSERT
- Case 1: Empty tree - Add root as black, simple
- Case 2: Parent is black - No properties violated, continue
- Case 3: Parent is red - No continguous reds, logically
the grandparent is black, now check the parent's siblings...
  - 3.1 Parent and Uncle are red - flip parent uncle grand-
        parent colors.
  - 3.2 Parent red, uncle black/NULL - single/double rotation,
    recolor needed

  - 3.2.1 Right, Right children - string of right red requires
    left rotation, Parent is now black, gp is now red

  - 3.2.2 Left, Left children - mirror above, right rotation,
    recolor parent to black, gp now red

  - 3.2.3 Parent right, child left - right rotation, perform 3.2.1

  - 3.2.4 Parent left, child right - left rotation perform 3.2.2


## AVL (Adelson, Veski, Landis) Approach

1. Perform the normal BST insertion. 
2. The current node must be one of the ancestors of the newly inserted node. Update the height of the current node. 
3. Get the balance factor (left subtree height – right subtree height) of the current node. 
4. If the balance factor is greater than 1, then the current node is unbalanced and we are either in the Left Left case or left Right case. To check whether it is left left case or not, compare the newly inserted key with the key in the left subtree root. 
5. If the balance factor is less than -1, then the current node is unbalanced and we are either in the Right Right case or Right-Left case. To check whether it is the Right Right case or not, compare the newly inserted key with the key in the right subtree root.  

