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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Self::help(vec![root], &mut res);
        res
    }

    fn help(arr: Vec<Option<Rc<RefCell<TreeNode>>>>, res: &mut Vec<i32>) {
        if arr.is_empty() {
            return;
        }
        let mut next_arr = vec![];
        arr.iter().for_each(|node| match node {
            Some(node) => {
                res.push(node.borrow().val);
                next_arr.push(node.borrow().left.clone());
                next_arr.push(node.borrow().right.clone());
            }
            None => (),
        });
        Self::help(next_arr, res);
    }
}

#[test]
fn check() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));

    assert_eq!(Solution::level_order(tree), vec![1, 2, 3, 4]);
}
