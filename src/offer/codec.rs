// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
struct Solution;
impl Solution {
    pub fn codec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut stat = vec![root];
        let mut res = vec![];
        while !stat.is_empty(){
            let mut next_stat = vec![];
            stat.iter().for_each(|node|{
                if let Some(node) = node {
                    res.push(Some(node.borrow().val));
                    next_stat.push(node.borrow().left.clone());
                    next_stat.push(node.borrow().right.clone());
                }
            });
            stat = next_stat;
        }
        res.iter().map(|i|{
            if let Some(i) = i {
                i.to_string()
            }else{
                "null".to_string()
            }
        }).collect::<Vec<_>>()
    }
}