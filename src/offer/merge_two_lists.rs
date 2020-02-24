// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1,mut l2) = (&l1,&l2);
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut nl = &mut dummy;
        loop{
            match(l1,l2){
                (Some(n1),Some(n2))=>{
                    let min = if n1.val <= n2.val{
                        l1 = &n1.next;
                        n1.val
                    }
                    else{
                        l2 = &n2.next;
                        n2.val
                    };
                    nl.as_mut().unwrap().next = Some(Box::new(ListNode{ 
                        val:min, next:None 
                    }));
                },
                (Some(n1),None)=>{
                    l1 = &n1.next;
                    nl.as_mut().unwrap().next = Some(Box::new(ListNode{ 
                        val:n1.val, next:None 
                    }));
                },
                (None,Some(n2))=>{
                    l2 = &n2.next;
                    nl.as_mut().unwrap().next = Some(Box::new(ListNode{ 
                        val:n2.val, next:None 
                    }));
                },
                _=>break,
            }
            nl = &mut nl.as_mut().unwrap().next;
        }
        dummy.unwrap().next
    }
}