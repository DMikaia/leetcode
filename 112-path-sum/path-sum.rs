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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(curr) = root {
            let curr_val = curr.borrow().val;
            
            if curr.borrow().left.is_none() && curr.borrow().right.is_none() {
                return curr_val == target_sum;
            }

            let left = curr.borrow().left.clone();
            let right = curr.borrow().right.clone();
            
            return Self::has_path_sum(left, target_sum - curr_val) 
                || Self::has_path_sum(right, target_sum - curr_val);
        }
        
        false
    }
}