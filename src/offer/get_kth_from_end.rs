#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn get_kth_from_end(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cur = &head;
        let mut len = 0;
        while let Some(node) = cur {
            len += 1;
            cur = &node.next;
        }
        let mut ptr = &mut head;
        for _ in 0..len - k {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        ptr.take()
    }
}
