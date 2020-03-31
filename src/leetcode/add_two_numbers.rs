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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head = None;
        let (mut l1, mut l2) = (&l1, &l2);
        loop {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    let mut new = n1.val + n2.val + carry;
                    if new > 9 {
                        carry = 1;
                        new = new - 10;
                    } else {
                        carry = 0;
                    }

                    head = Some(Box::new(ListNode {
                        next: head,
                        val: new,
                    }));
                    l1 = &n1.next;
                    l2 = &n2.next;
                }
                (Some(n1), None) => {
                    let mut new = n1.val + carry;
                    if new > 9 {
                        carry = 1;
                        new = new - 10;
                    } else {
                        carry = 0;
                    }
                    head = Some(Box::new(ListNode {
                        next: head,
                        val: new,
                    }));
                    l1 = &n1.next;
                    l2 = &None;
                }
                (None, Some(n2)) => {
                    let mut new = n2.val + carry;
                    if new > 9 {
                        carry = 1;
                        new = new - 10;
                    } else {
                        carry = 0;
                    }
                    head = Some(Box::new(ListNode {
                        next: head,
                        val: new,
                    }));
                    l1 = &None;
                    l2 = &n2.next;
                }
                (None, None) if carry == 1 => {
                    head = Some(Box::new(ListNode { next: head, val: 1 }));
                }
                _ => break,
            }
        }
        head
    }

    fn sum(
        l1: &Option<Box<ListNode>>,
        l2: &Option<Box<ListNode>>,
        mut carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                let mut new = n1.val + n2.val + carry;
                if new > 9 {
                    carry = 1;
                    new = new - 10;
                } else {
                    carry = 0;
                }
                Some(Box::new(ListNode {
                    next: Self::sum(&n1.next, &n2.next, carry),
                    val: new,
                }))
            }
            (Some(n1), None) => {
                let mut new = n1.val + carry;
                if new > 9 {
                    carry = 1;
                    new = new - 10;
                } else {
                    carry = 0;
                }
                Some(Box::new(ListNode {
                    next: Self::sum(&n1.next, &None, carry),
                    val: new,
                }))
            }
            (None, Some(n2)) => {
                let mut new = n2.val + carry;
                if new > 9 {
                    carry = 1;
                    new = new - 10;
                } else {
                    carry = 0;
                }
                Some(Box::new(ListNode {
                    next: Self::sum(&None, &n2.next, carry),
                    val: new,
                }))
            }
            (None, None) if carry == 1 => Some(Box::new(ListNode { next: None, val: 1 })),
            _ => None,
        }
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode::new(1))),
            Some(Box::new(ListNode::new(2)))
        ),
        Some(Box::new(ListNode::new(3)))
    );
}
