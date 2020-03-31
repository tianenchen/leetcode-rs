use crate::util::tree::*;

use std::rc::Rc;
use std::cell::RefCell;
struct Solution;
impl Solution {
    // pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    //     Solution::flatten_helper(root.clone());
    // }

    // fn flatten_helper(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     if let Some(node) = root {
    //         let tail = Solution::flatten_helper(node.borrow().left.clone());
    //         Solution::flatten_helper(node.borrow_mut().right.clone());
    //         let mut right = node.borrow().right.clone();
    //         let mut ptr = node.clone();
    //         if let Some(tail) = tail {
    //             let head = node.borrow_mut().left.take();
    //             node.borrow_mut().right = head;
    //             tail.borrow_mut().right = right.clone();
    //             ptr = tail.clone();
    //         }
    //         while let Some(next) = ptr.clone().borrow().right.clone() {
    //             ptr = next
    //         }
    //         Some(ptr)
    //     } else {
    //         None
    //     }
    // }
}