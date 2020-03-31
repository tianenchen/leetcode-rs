#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &head;
        let mut cap = 0;
        while let Some(node) = cur {
            cap += 1;
            cur = &node.next;
        }
        let mid = cap >> 1;
        let mut cur = head;
        for _ in 0..mid {
            cur = cur.unwrap().next;
        }
        cur
    }
}

#[test]
fn check() {
    assert_eq!(2 >> 1, 1);
    assert_ne!(2 >> 1 + 1, 2); //Attention
    assert_eq!((2 >> 1) + 1, 2);
}
