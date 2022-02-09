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

fn remove(mut head: &mut Option<Box<ListNode>>, index: usize) -> Option<i32> {
    let mut n = index;

    while n > 0 {
        head = match head.as_mut() {
            None => return None,
            Some(node) => &mut node.next,
        };
        n -= 1;
    }

    match head.take().map(|x| *x) {
        Some(ListNode { val, next }) => {
            *head = next;
            Some(val)
        }
        None => None,
    }
}

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut i = 0;
        let mut last_node = &head;

        while let Some(node) = last_node {
            last_node = &node.next;
            i += 1;
        }

        remove(&mut head, i - n as usize);
        head
    }
}