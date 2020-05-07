use crate::util::list::*;
struct Solution;

impl Solution {
    fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(-1)));
        let mut c = &mut head;
        while l1.as_ref().is_some() && l2.as_ref().is_some() {
            if &l1.as_ref().unwrap().val < &l2.as_ref().unwrap().val {
                c.as_mut().unwrap().next = Some(Box::new(ListNode::new(l1.as_ref().unwrap().val)));
                l1 = l1.as_mut().unwrap().next.take();
            } else {
                c.as_mut().unwrap().next = Some(Box::new(ListNode::new(l2.as_ref().unwrap().val)));
                l2 = l2.as_mut().unwrap().next.take();
            }
            c = &mut c.as_mut().unwrap().next;
        }
        if l1.is_some() {
            c.as_mut().unwrap().next = l1;
        } else {
            c.as_mut().unwrap().next = l2;
        }
        head.unwrap().next
    }
}
