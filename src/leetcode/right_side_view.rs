struct Solution;

use crate::util::tree::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut deq = std::collections::VecDeque::new();
        deq.push_back(vec![root.as_ref().unwrap().clone()]);
        let mut res = vec![];
        while let Some(line) = deq.pop_front() {
            let mut next_line = vec![];
            let len = line.len();
            for i in 0..len {
                //the last
                if i == len - 1 {
                    res.push(line[i].borrow().val);
                }
                if let Some(left) = &line[i].borrow().left {
                    next_line.push(left.clone());
                }
                if let Some(right) = &line[i].borrow().right {
                    next_line.push(right.clone());
                }
            }
            if !next_line.is_empty() {
                deq.push_back(next_line);
            }
        }
        res
    }
}
