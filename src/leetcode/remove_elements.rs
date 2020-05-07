use crate::util::list::*;

struct Solution;
impl Solution {

    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(-1)));
        let mut cur = dummy.as_mut();
        while let Some(mut n) = head{
            head = n.next.take();
            if n.val != val{
                cur.as_mut().unwrap().next = Some(n);
                cur = cur.unwrap().next.as_mut();
            }
        }
        dummy.unwrap().next
    }
    pub fn remove_elements2(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while cur.is_some(){
            if cur.as_ref().unwrap().val == val{
                let mut tmp = cur.take();
                std::mem::swap(cur, &mut tmp.as_mut().unwrap().next);
                continue
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::remove_elements(as_list(vec![1, 2]), 2),
        as_list(vec![1])
    );
    assert_eq!(
        Solution::remove_elements(as_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
        as_list(vec![1, 2, 3, 4, 5])
    );
}
