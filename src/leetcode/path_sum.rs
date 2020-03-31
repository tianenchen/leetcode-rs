use crate::util::tree::*;

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::help(&root, sum, 0, vec![], &mut res);
        res
    }

    fn help(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        cur: i32,
        mut his: Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = node {
            let val = node.borrow().val;
            let new_cur = cur + val;
            his.push(val);
            if new_cur == target && node.borrow().left.is_none() && node.borrow().right.is_none() {
                res.push(his.clone());
                return;
            }
            Self::help(&node.borrow().left, target, new_cur, his.clone(), res);
            Self::help(&node.borrow().right, target, new_cur, his, res);
        }
    }
}

/*

给定一个二叉树和一个目标和，找到所有从根节点到叶子节点路径总和等于给定目标和的路径。

说明: 叶子节点是指没有子节点的节点。

示例:
给定如下二叉树，以及目标和 sum = 22，

              5
             / \
            4   8
           /   / \
          11  13  4
         /  \    / \
        7    2  5   1
返回:

[
   [5,4,11,2],
   [5,8,4,5]
]

*/
