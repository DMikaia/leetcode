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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_rc) = root {
            let mut node = node_rc.borrow_mut();

            let tmp = node.left.take();
            node.left = node.right.take();
            node.right = tmp;

            if let Some(left) = node.left.clone() {
                Self::invert_tree(Some(left.clone()));
            }

            if let Some(right) = node.right.clone() {
                Self::invert_tree(Some(right.clone()));
            }

            Some(node_rc.clone())
        } else {
            None
        }
    }
}