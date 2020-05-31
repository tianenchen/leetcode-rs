use crate::util::tree::*;
use std::rc::Rc;
use std::cell::RefCell;

/*给定两个非空二叉树 s 和 t，检验 s 中是否包含和 t 具有相同结构和节点值的子树。s 的一个子树包括 s 的一个节点和这个节点的所有子孙。s 也可以看做它自身的一棵子树。

示例 1:
给定的树 s:

     3
    / \
   4   5
  / \
 1   2
给定的树 t：

   4 
  / \
 1   2
返回 true，因为 t 与 s 的一个子树拥有相同的结构和节点值。

示例 2:
给定的树 s：

     3
    / \
   4   5
  / \
 1   2
    /
   0
给定的树 t：

   4
  / \
 1   2
返回 false。

*/
struct Solution;
impl Solution {
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::help(&s,&t)
    }

    fn help(node:&Option<Rc<RefCell<TreeNode>>> , t :&Option<Rc<RefCell<TreeNode>>>)->bool{
        if *node == *t{
            true
        }
        else if let Some(node) = node{
            Self::help(&node.borrow().left.clone(), t)
                ||Self::help(&node.borrow().right.clone(), t)
        }
        else{
            false
        }
    }
}