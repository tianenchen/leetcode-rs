#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn as_list(l:Vec<i32>)->Option<Box<ListNode>>{
    let mut node = None;
    for i in l.into_iter().rev(){
        node = Some(Box::new(ListNode{
            next:node,
            val:i,
        }));
    }
    node
}

#[test]
fn check() {
    let v = vec![1,2,3,4,5,6];
    let l = as_list(v.clone());
    let mut cur = &l;
    for i in v{
        assert_eq!(i,cur.as_ref().unwrap().val);
        cur = &cur.as_ref().unwrap().next;
    }
}