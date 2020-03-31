use crate::util::tree::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::help(&root, None, None)
    }
    fn help(root: &Option<Rc<RefCell<TreeNode>>>, low: Option<i32>, upper: Option<i32>) -> bool {
        if let Some(node) = root {
            let val = node.borrow().val;
            if low.is_some() && val <= low.unwrap() {
                return false;
            }
            if upper.is_some() && val >= upper.unwrap() {
                return false;
            }
            Self::help(&node.borrow().left, low, Some(val))
                && Self::help(&node.borrow().right, Some(val), upper)
        } else {
            true
        }
    }
}
