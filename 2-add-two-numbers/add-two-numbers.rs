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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>, 
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut tail = result.as_mut();
        let mut carry: i32 = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let x = l1.as_ref().map_or(0, |node| node.val);
            let y = l2.as_ref().map_or(0, |node| node.val);
            let sum = carry + x + y;

            carry = sum / 10;

            let new_node = Box::new(ListNode {
                val: sum % 10,
                next: None,
            });

            if let Some(t) = tail {
                t.next = Some(new_node);
                tail = t.next.as_mut();
            }

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        result.unwrap().next
    }
}
