// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;
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
pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::tree(
            0,
            preorder.len() as i32 - 1,
            0,
            inorder.len() as i32 - 1,
            &preorder,
            &inorder,
        )
    }
    fn tree(
        pl: i32,
        pr: i32,
        ml: i32,
        mr: i32,
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pl > pr || ml > mr {
            return None;
        }
        let root = preorder[pl as usize];
        let root_idx = ml
            + inorder[ml as usize..=mr as usize]
                .iter()
                .position(|x| *x == root)
                .unwrap() as i32;
        let tree = Rc::new(RefCell::new(TreeNode {
            val: root,
            left: Self::tree(
                pl + 1,
                root_idx - ml + pl,
                ml,
                root_idx - 1,
                preorder,
                inorder,
            ),
            right: Self::tree(
                root_idx - ml + pl + 1,
                pr,
                root_idx + 1,
                mr,
                preorder,
                inorder,
            ),
        }));
        Some(tree)
    }
}

/*

前序遍历 preorder = [3,9,20,15,7]
中序遍历 inorder = [9,3,15,20,7]

        3     9 20 15 7            =>          9            20    15    7
       root                                   root         root
        9   3   15 20 7            =>          9            15    20    7
           root                                            left       right

*/
