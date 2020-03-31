struct Solution;

// Definition for singly-linked list.
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

#[allow(dead_code)]
impl Solution {
    #[cfg(feature = "unsafe")]
    pub fn delete_node2(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        unsafe {
            let mut head = Some(Box::new(ListNode { val: 0, next: head }));
            let mut cur: *const Option<Box<ListNode>> = &mut head; // 一个const指针  其实就相当于引用
            let mut new_cur: *mut Option<Box<ListNode>> = &mut head; // 一个 mut 裸执政  相当于可变引用
            let mut idx = 0;
            while let Some(node) = &*cur {
                //接引用裸指针
                if node.val == val && idx > 0 {
                    break;
                }
                idx += 1;
                cur = &node.next;
            }
            for _ in 0..idx - 1 {
                new_cur = &mut (*new_cur).as_mut().unwrap().next;
            }
            let next = (*new_cur)
                .as_mut()
                .unwrap()
                .next
                .as_mut()
                .unwrap()
                .next
                .take();
            (*new_cur).as_mut().unwrap().next = next;
            head.unwrap().next
        }
    }

    #[cfg(not(feature = "unsafe"))]
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut cur = &head;
        let mut idx = 0;

        while let Some(node) = cur {
            if node.val == val && idx > 0 {
                break;
            }
            idx += 1;
            cur = &node.next;
        }

        let mut new_cur = head.as_mut();
        for _ in 0..idx - 1 {
            new_cur = new_cur.unwrap().next.as_mut();
        }
        let next = new_cur.as_mut().unwrap().next.as_mut().unwrap().next.take();
        new_cur.as_mut().unwrap().next = next;
        head.unwrap().next
    }
}

// [2,0,1,3]  =>   [0,2,0,1,3]
