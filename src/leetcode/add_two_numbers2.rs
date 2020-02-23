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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>,l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (len1,len2) = (Self::len(&l1),Self::len(&l2));
        let (carry,res) = if len1 > len2{
            Self::calc(&l1,&l2,len1-len2)
        }
        else if len1 < len2{
            Self::calc(&l2,&l1,len2-len1)
        }
        else{
            Self::calc(&l1,&l2,0)
        };
        if carry == 1{
            return Some(Box::new(ListNode{
                next:res,val:1
            }))
        }
        res
    }

    fn calc(long: &Option<Box<ListNode>>,short: &Option<Box<ListNode>>,mut diff:i32)->(i32,Option<Box<ListNode>>){
        if let Some(node) = long {
            diff -= 1;
            let carry;
            let (next,mut val) = if diff >=0{
                let (carry,next) = Self::calc(&node.next,short,diff);
                (next,node.val+carry)
            }else{
                let short = short.as_ref().unwrap();
                let (carry,next) = Self::calc(&node.next,&short.next,diff);
                (next,node.val+carry+&short.val)
            };
            if val > 9{
                carry = 1;
                val -= 10;
            }else{
                carry = 0;
            }
            (carry,Some(Box::new(ListNode{
                next,val
            })))
        }else{
            (0,None)
        }
    }

    fn len(list: &Option<Box<ListNode>>)->i32{
        match list{
            Some(list) => 1+Self::len(&list.next),
            None=>0,
        }
    }
}

#[test]
fn check() {
    assert_eq!(Solution::add_two_numbers(Some(Box::new(ListNode::new(1))),Some(Box::new(ListNode::new(2)))),
                                        Some(Box::new(ListNode::new(3))));
}