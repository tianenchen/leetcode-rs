

use crate::util::tree::*;
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut sorted = vec![];
        Self::help(root, &mut sorted);
        let mut x = None;
        let mut y = None;
        for (i,node) in sorted[0..sorted.len()-1].iter().enumerate(){
            if sorted[i+1].borrow().val < node.borrow().val{
                y = Some(sorted[i+1].clone());
                if x.is_none(){
                    x=Some(node);
                }
                else{
                    break;
                }
            }
        }
        let tmp = y.as_ref().unwrap().borrow().val;
        y.as_mut().unwrap().borrow_mut().val = x.as_ref().unwrap().borrow().val;
        x.as_mut().unwrap().borrow_mut().val = tmp;
    }

    fn help(node :&mut Option<Rc<RefCell<TreeNode>>>,sorted :&mut Vec<Rc<RefCell<TreeNode>>>){
        if node.is_some(){
            Self::help(&mut node.as_mut().unwrap().borrow_mut().left, sorted);
            sorted.push(node.as_ref().unwrap().clone());
            Self::help(&mut node.as_mut().unwrap().borrow_mut().right, sorted);
        }
    }
}

#[test]
fn check() {
    let mut tree = as_tree(vec![Some(2),None,Some(1)]);
    Solution::recover_tree(&mut tree);
    assert_eq!(tree,as_tree(vec![Some(1),None,Some(2)]));
}