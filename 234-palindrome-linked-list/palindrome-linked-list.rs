// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut mid = head.as_ref();
        let mut fast = head.as_ref();

        let mut stack = Vec::new();
        while let Some(ref mid_el) = mid {
            fast = match fast {
                Some(node) => match &node.next {
                    Some(next_node) => match &next_node.next {
                        Some(second_next_node) => Some(second_next_node), 
                        None => None,
                    },
                    None => None,
                },
                None => None,
            };

            if fast.is_none() && mid_el.next.is_some()  {
                stack.push(mid_el.next.as_ref().unwrap().val);
            }

            mid = mid_el.next.as_ref();
        }
        drop(fast);
        drop(mid);

        let mut list = head.as_ref();
        while let (Some(ref node), Some(expected)) = (list, stack.pop()) {
            if node.val != expected {
                return false;
            }

            list = node.next.as_ref();
        }
        drop(list);

        stack.is_empty()
    }
}