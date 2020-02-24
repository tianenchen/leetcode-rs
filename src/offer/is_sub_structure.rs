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
    pub fn is_sub_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::help(&a, &b, false)
    }
    fn help(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>,comparing:bool)->bool{
        if b.is_none(){
            comparing
        }
        else if a.is_none(){
            false
        }
        else{
            let va = (*a.as_ref().unwrap()).borrow().val;
            let vb = (*b.as_ref().unwrap()).borrow().val;
            if comparing{
                if va==vb{
                    Self::help(&(*a.as_ref().unwrap()).borrow().left,
                    &(*b.as_ref().unwrap()).borrow().left,comparing)&&
                    Self::help(&(*a.as_ref().unwrap()).borrow().right,
                    &(*b.as_ref().unwrap()).borrow().right,comparing)
                }
                else{
                    false
                }
            }
            else{
                if va == vb{
                    Self::help(&(*a.as_ref().unwrap()).borrow().left,
                    &(*b.as_ref().unwrap()).borrow().left,true)&&
                    Self::help(&(*a.as_ref().unwrap()).borrow().right,
                    &(*b.as_ref().unwrap()).borrow().right,true)
                }
                else{
                    Self::help(&(*a.as_ref().unwrap()).borrow().left,
                    b,comparing)||
                    Self::help(&(*a.as_ref().unwrap()).borrow().right,
                    b,comparing)
                }
            }
        }
    }
}