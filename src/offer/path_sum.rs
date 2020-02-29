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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::help(&root,sum, vec![0], &mut res);
        res
    }

    fn help(root: &Option<Rc<RefCell<TreeNode>>>,sum :i32,mut crumbs:Vec<i32>,mut res: &mut Vec<Vec<i32>>){
        if let Some(node) = root {
            let val = node.borrow().val;
            let curr = crumbs.last().unwrap()+val;
            if node.borrow().left.is_none() && node.borrow().right.is_none(){
                if curr == sum{
                    *crumbs.last_mut().unwrap() = val;
                    res.push(crumbs);
                }
            }
            else{
                *crumbs.last_mut().unwrap() = val;
                crumbs.push(curr);
                Self::help(&node.borrow().left, sum, crumbs.clone(), &mut res);
                Self::help(&node.borrow().right, sum, crumbs.clone(), &mut res);
            }
        }
    }
}

#[test]
fn check() {
    // let mut v = vec![1];
    // *v.last_mut().unwrap() = 11;
    // println!(" last : {}", v.last().unwrap());
}