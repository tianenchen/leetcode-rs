use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree::*;

struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            Self::helper(&root.borrow().left,&root.borrow().right)
        }
        else{
            true
        }
    }

    fn helper(l:&Option<Rc<RefCell<TreeNode>>>,r:&Option<Rc<RefCell<TreeNode>>>)->bool{
        match (l,r){
            (Some(l),Some(r))=>{
                if l.borrow().val == r.borrow().val{
                    Self::helper(&l.borrow().left, &r.borrow().right) && 
                    Self::helper(&l.borrow().right, &r.borrow().left)
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