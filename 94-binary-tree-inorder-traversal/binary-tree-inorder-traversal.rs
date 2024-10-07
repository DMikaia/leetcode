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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![];
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            while let Some(node) = cur {
                stack.push(node.clone());
                cur = node.borrow_mut().left.clone();
            }
            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);
                cur = node.borrow_mut().right.clone();
            }
        }
        result
    }
}