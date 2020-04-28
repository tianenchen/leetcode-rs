use crate::util::tree::*;

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = vec![];
        Self::help(&root, String::new(), &mut res);
        res
    }

    fn help(root: &Option<Rc<RefCell<TreeNode>>>, mut path: String, res: &mut Vec<String>) {
        if let Some(root) = root {
            if path.is_empty() {
                path = root.borrow().val.to_string();
            } else {
                path = format!("{}->{}", path, root.borrow().val);
            }
            if root.borrow().left.is_none() && root.borrow().right.is_none() {
                res.push(path);
            } else {
                Self::help(&root.borrow().left, path.clone(), res);
                Self::help(&root.borrow().right, path.clone(), res);
            }
        }
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::binary_tree_paths(as_tree(vec![Some(1), Some(2), Some(3), None, Some(5)])),
        vec!["1->2->5", "1->3"].to_owned()
    );
}
