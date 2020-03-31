use crate::util::tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut deq = std::collections::VecDeque::new();
        deq.push_back(vec![root]);
        let mut count = 0;
        let mut res = vec![];
        while let Some(nodes) = deq.pop_front() {
            let mut next = vec![];
            let mut tmp = vec![];
            for node in nodes {
                if let Some(node) = node {
                    tmp.push(node.borrow().val);
                    next.push(node.borrow().left.clone());
                    next.push(node.borrow().right.clone());
                }
            }
            if next.is_empty() {
                break;
            }
            if count % 2 == 1 {
                tmp.reverse();
            }
            deq.push_back(next);
            res.push(tmp);
            count += 1;
        }
        res
    }
}
