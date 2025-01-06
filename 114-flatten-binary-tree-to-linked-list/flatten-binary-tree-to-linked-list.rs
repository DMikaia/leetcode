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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        if let Some(root_node) = root {
            stack.push(root_node.clone());
        }

        while let Some(node) = stack.pop() {
            if let Some(right) = node.borrow_mut().right.take() {
                stack.push(right);
            }
            if let Some(left) = node.borrow_mut().left.take() {
                stack.push(left);
            }

            node.borrow_mut().left = None;

            if let Some(prev) = prev {
                prev.borrow_mut().right = Some(node.clone());
            } else {
                *root = Some(node.clone()); 
            }

            prev = Some(node.clone());
        }
    }
}