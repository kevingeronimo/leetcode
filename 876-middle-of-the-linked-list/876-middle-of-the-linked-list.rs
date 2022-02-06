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
use std::iter::Iterator;

struct NodeIter<'a> {
    node: &'a Option<Box<ListNode>>,
}

impl<'a> NodeIter<'a> {
    fn new(node: &'a Option<Box<ListNode>>) -> Self {
        NodeIter { node }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Box<ListNode>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.node {
            Some(node) => {
                let current_node = &node;
                self.node = &node.next;
                Some(current_node)
            }
            None => None,
        }
    }
}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let last_node = NodeIter::new(&head).enumerate().last();

        match last_node {
            Some((i, _)) => {
                let mid = if i % 2 == 0 { i / 2 } else { i / 2 + 1 };
                NodeIter::new(&head).nth(mid).cloned()
            }
            None => None,
        }
    }
}