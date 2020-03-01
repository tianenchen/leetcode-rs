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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Self::help(&root,&mut 0)
        let mut flag = true;
        Self::help2(&root,&mut flag);
        flag
    }


    fn help(node :&Option<Rc<RefCell<TreeNode>>>,step :&mut i32)->bool{
        if let Some(node) = node {
            *step +=1;
            let (mut left,mut right) = (*step,*step);
            if !Self::help(&node.borrow().left,&mut left)|| !Self::help(&node.borrow().right,&mut right){
                return false
            }
            *step = std::cmp::max(left,right);
            i32::abs(left-right) < 2
        }
        else {
            true
        }
    }

    fn help2(node  :&Option<Rc<RefCell<TreeNode>>>,flag :&mut bool) -> i32{
        if let Some(node) = node {
            let left = Self::help2(&node.borrow().left,flag);
            let right = Self::help2(&node.borrow().right,flag);
            if i32::abs(left-right) > 1{
                *flag = false;
            }
            1+std::cmp::max(left,right)
        }
        else{
            0
        }
    }
}