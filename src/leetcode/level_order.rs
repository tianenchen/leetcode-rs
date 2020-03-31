use crate::util::tree::*;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut deq = std::collections::VecDeque::new();
        let mut res = vec![];
        deq.push_back(vec![root]);
        while let Some(nodes) = deq.pop_front() {
            let mut tmp = vec![];
            let mut next = vec![];
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
            res.push(tmp);
            deq.push_back(next);
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::level_order(as_tree(vec![Some(1), Some(2), Some(3)])),
        vec![vec![1], vec![2, 3]]
    )
}

/*

给定一个二叉树，返回其按层次遍历的节点值。 （即逐层地，从左到右访问所有节点）。

例如:
给定二叉树: [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7
返回其层次遍历结果：

[
  [3],
  [9,20],
  [15,7]
]

*/
