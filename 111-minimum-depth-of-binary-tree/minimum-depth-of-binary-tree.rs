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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(node) = root else {
            return 0;
        };
        let left_value = Self::min_depth(node.borrow().left.clone());
        let right_value = Self::min_depth(node.borrow().right.clone());

        if left_value == 0 && right_value > 0 {
                right_value + 1 
        } else if left_value > 0 && right_value == 0 {
                left_value + 1
        } else {
            left_value.min(right_value) + 1
        }
    }
}