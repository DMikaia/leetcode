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
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;
        let mut carry: i32 = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let x = l1.as_ref().map_or(0, |node| node.val);
            let y = l2.as_ref().map_or(0, |node| node.val);
            let sum = carry + x + y;

            carry = sum / 10;

            let new_node = tail.insert(Box::new(ListNode {
                val: sum % 10,
                next: None,
            }));

            tail = &mut new_node.next;

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        head
    }
}
