// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
#[allow(dead_code)]
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
#[allow(dead_code)]
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            Self::help(&root.borrow().left,&root.borrow().right)
        }
        else{
            true
        }
    }
    fn help(left :&Option<Rc<RefCell<TreeNode>>>,right :&Option<Rc<RefCell<TreeNode>>>) -> bool{
        match(left,right){
            (Some(l),Some(r))=>{
                if l.borrow().val == r.borrow().val{
                    Self::help(&l.borrow().left,&r.borrow().right)&&
                    Self::help(&l.borrow().right,&r.borrow().left)
                }
                else{
                    false
                }
            },
            (None,None)=>true,
            _=>false,
        }
    }
}