use crate::util::tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::help(&root, false)
    }

    fn help(root: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(node) = root {
            //有左节点
            if is_left && node.borrow().left.is_none() && node.borrow().right.is_none() {
                return node.borrow().val;
            }
            Self::help(&node.borrow().left, true) + Self::help(&node.borrow().right, false)
        } else {
            0
        }
    }
}
