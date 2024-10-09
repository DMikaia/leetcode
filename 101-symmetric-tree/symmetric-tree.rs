// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            return Self::is_nodes_symetric(node.borrow().left.clone(),node.borrow().right.clone());
        }

        false
    }

    pub fn is_nodes_symetric(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();

                if left.val != right.val {
                    return false;
                }

                let next_left = Self::is_nodes_symetric(left.left.clone(), right.right.clone());
                let next_right = Self::is_nodes_symetric(left.right.clone(), right.left.clone());

                next_left && next_right
            },
            (_) => false
        }
    }
}