use std::cell::RefCell;
use std::rc::Rc;

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

pub fn as_tree(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let head = Some(Rc::new(RefCell::new(TreeNode::new(v[0].unwrap()))));
    let mut deq = std::collections::VecDeque::new();
    deq.push_back(head.as_ref().unwrap().clone());
    for children in v[1..].chunks(2) {
        let parent = deq.pop_front().unwrap();
        if let Some(v) = children[0] {
            let tmp = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            parent.borrow_mut().left = tmp;
            deq.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 && children[1].is_some() {
            let tmp = Some(Rc::new(RefCell::new(TreeNode::new(children[1].unwrap()))));
            parent.borrow_mut().right = tmp;
            deq.push_back(parent.borrow().right.as_ref().unwrap().clone());
        }
    }
    head
}

#[test]
fn tree_test() {
    assert_eq!(
        as_tree(vec![Some(1), None, Some(2)]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })))
        })))
    );
    assert_eq!(
        as_tree(vec![Some(1), Some(2), Some(3)]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })))
        })))
    );
}
