#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
#[allow(dead_code)]
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
#[allow(dead_code)]
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut node = root;
        let mut stack = std::collections::LinkedList::default();
        let mut res = vec![];
        while node.is_some() || !stack.is_empty() {
            if let Some(inner) = node.clone() {
                stack.push_back(node.clone()); //当前节点入栈
                if inner.borrow().left.is_some() {
                    //左子树有值
                    node = inner.borrow().left.clone();
                } else {
                    //没有左子树了
                    res.push(inner.borrow().val);
                    if let Some(before) = stack.pop_back() {
                        node = before.unwrap().borrow().right.clone();
                    }
                }
            } else if let Some(before) = stack.pop_back() {
                res.push(before.as_ref().unwrap().borrow().val);
                node = before.unwrap().borrow().right.clone();
            }
        }
        // let mut res = vec![];
        // Self::help(&root,&mut res);
        res
    }

    fn help(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::help(&node.borrow().left, res);
            res.push(node.borrow().val);
            Self::help(&node.borrow().right, res);
        }
    }
}

//         1

//     2       3

// 4     5   6    7
