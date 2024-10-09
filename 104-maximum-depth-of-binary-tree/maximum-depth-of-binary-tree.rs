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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut curr_node = node.borrow_mut();
            let mut max = 1; 

            let left_max = Self::max_depth(curr_node.left.take());
            let right_max = Self::max_depth(curr_node.right.take());

            if left_max + 1 > right_max + 1 {
                left_max + 1
            } else {
                right_max + 1
            }
        } else {
            0
        }
    }
}