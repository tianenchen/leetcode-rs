// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::help(&root,0)
    }
    fn help(node: &Option<Rc<RefCell<TreeNode>>>,step :i32)->i32{
        if let Some(node) = node {
            std::cmp::max(Self::help(&node.borrow().left,step+1),Self::help(&node.borrow().right,step+1))
        }
        else{
            step
        }
    }
}