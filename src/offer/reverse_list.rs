// Definition for singly-linked list.
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
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::help(&head,None)
    }

    fn help(node: &Option<Box<ListNode>>,before:Option<Box<ListNode>>)-> Option<Box<ListNode>>{
        match node{
            Some(node)=>{
                Self::help(&node.next,Some(Box::new(ListNode{
                    val:node.val,
                    next:before,
                })))
            },
            None=>before,
        }
    }
}