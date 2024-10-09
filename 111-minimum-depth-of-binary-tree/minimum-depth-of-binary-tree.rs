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
use std::cmp;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_node = Self::min_depth(node.borrow().left.clone());
            let right_node = Self::min_depth(node.borrow().right.clone());

            if left_node == 0 && right_node > 0 {
                right_node + 1 
            } else if left_node > 0 && right_node == 0 {
                left_node + 1
            } else {
                cmp::min(left_node, right_node) + 1
            }
        } else {
            0
        }
    }
}