use crate::util::list::*;
struct Solution;
impl Solution {

    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        lists.into_iter().fold(None,Self::merge_two_lists)
    }

    fn merge_two_lists(mut l:Option<Box<ListNode>>,mut r:Option<Box<ListNode>>)->Option<Box<ListNode>>{
        let mut head = Some(Box::new(ListNode::new(-1)));  
        let mut c = &mut head;
        while l.as_ref().is_some() && r.as_ref().is_some(){
            if &l.as_ref().unwrap().val < &r.as_ref().unwrap().val{
                c.as_mut().unwrap().next = Some(Box::new(ListNode::new(l.as_ref().unwrap().val)));
                l = l.as_mut().unwrap().next.take();
            }
            else{
                c.as_mut().unwrap().next = Some(Box::new(ListNode::new(r.as_ref().unwrap().val)));
                r = r.as_mut().unwrap().next.take();
            }
            c = &mut c.as_mut().unwrap().next;
        }
        if l.is_some(){
            c.as_mut().unwrap().next = l;
        }
        else{
            c.as_mut().unwrap().next = r;
        }
        head.unwrap().next
    }

}