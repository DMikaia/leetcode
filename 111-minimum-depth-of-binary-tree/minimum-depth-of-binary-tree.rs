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
use std::collections::VecDeque;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0
        };

        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut depth = 1;

        while !queue.is_empty() {
            let level_size = queue.len();

            for _ in 0..level_size {
                let node_rc = queue.pop_front().unwrap();
                let node = node_rc.borrow();

                if node.left.is_none() && node.right.is_none() {
                    return depth;
                }

                if let Some(left) = &node.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node.right {
                    queue.push_back(right.clone());
                }
            }
            
            depth += 1;
        }

        depth
    }
}