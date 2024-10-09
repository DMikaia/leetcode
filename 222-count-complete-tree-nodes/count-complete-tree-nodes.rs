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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            return Self::count_current_nodes(Some(node), 0);
        }

        0
    }

    pub fn count_current_nodes(root: Option<Rc<RefCell<TreeNode>>>, curr_val: i32) -> i32 {
        if let Some(node) = root {
            let value = curr_val + 1;

            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            let left_counter = Self::count_current_nodes(left, 0);
            let right_counter = Self::count_current_nodes(right, 0);

            return value + left_counter +right_counter;
        }

        curr_val
    }
}