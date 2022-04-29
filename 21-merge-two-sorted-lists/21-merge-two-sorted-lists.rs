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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode {next: None, val: 0}));
        let mut tail = &mut head;
        let mut list1 = list1;
        let mut list2 = list2;
        loop {
            let old_tail =  tail.as_mut().unwrap();
            let j = match (list1.take(), list2.take()) {
                (Some(mut x), Some(mut y)) => {
                    if x.val > y.val { list2 = y.next.take(); list1 = Some(x); y} else { list2 = Some(y);list1 = x.next.take(); x}
                },
                (Some(mut x), None) | (None, Some(mut x))  => {  old_tail.next = Some(x); break },

                (None, None) => break
            };
            old_tail.next = Some(j);
            tail = &mut old_tail.next;
        }
        head.unwrap().next
    }  
}