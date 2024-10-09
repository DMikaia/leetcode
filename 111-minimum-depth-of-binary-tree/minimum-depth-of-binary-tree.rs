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
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        let left = Self::min_depth(left);
        let right = Self::min_depth(right);

        if left == 0 && right > 0 {
                right + 1 
        } else if left > 0 && right == 0 {
                left + 1
        } else {
            left.min(right) + 1
        }
    }
}