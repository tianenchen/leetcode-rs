use crate::util::tree::*;

use std::rc::Rc;
use std::cell::RefCell;
struct Solution;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::help(0,inorder.len(), 0, postorder.len(), &inorder,&postorder)
    }
    fn help(il:usize,ir:usize,pl:usize,pr:usize,inorder: &Vec<i32>, postorder: &Vec<i32>)->Option<Rc<RefCell<TreeNode>>>{
        if il<ir && pl<pr{
            let root = postorder[pr-1];
            let root_idx_in_inorder = inorder.iter().position(|x|*x==root).unwrap();
            let left_tree_len = root_idx_in_inorder - il;
            Some(Rc::new(RefCell::new(TreeNode{
                val:root,
                left:Self::help(il,root_idx_in_inorder,pl,pl+left_tree_len,inorder,postorder),
                right:Self::help(root_idx_in_inorder+1,ir,pl+left_tree_len,pr-1,inorder,postorder),
            })))
        }
        else{
            None
        }
    }
}
/*

根据一棵树的中序遍历与后序遍历构造二叉树。

注意:
你可以假设树中没有重复的元素。

例如，给出

前序遍历 根左右
中序遍历 inorder = [9,3,15,20,7]左根右
后序遍历 postorder = [9,15,7,20,3]左右根
返回如下的二叉树：

    3
   / \
  9  20
    /  \
   15   7

*/