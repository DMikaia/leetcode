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
        match root {
            Some(root) => {
                let mut root = root.borrow_mut();
                
                Self::check_nodes(root.left.take(), root.right.take())
            }
            None => false
        }
    }

    pub fn check_nodes(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let mut left = left.borrow_mut();
                let mut right = right.borrow_mut();

                if left.val != right.val {
                    return false;
                }

                return Self::check_nodes(left.left.take(), right.right.take()) &&
                Self::check_nodes(left.right.clone(), right.left.take());
            },
            (_) => false
        }
    }
}