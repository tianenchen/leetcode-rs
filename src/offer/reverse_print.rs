struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
#[allow(dead_code)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
#[allow(dead_code)]
impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        Self::recur(&head)
    }

    fn recur(node : &Option<Box<ListNode>>) -> Vec<i32>{
        match node{
            Some(child) => {
                let mut res = Self::recur(&child.next);
                res.push(child.val);
                res
            },
            None => vec![]
        }
    }
}