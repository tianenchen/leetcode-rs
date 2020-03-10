use std::rc::Rc;
use std::cell::RefCell;

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
            right: None
        }
    }
}

pub fn as_tree(v:Vec<Option<i32>>)->Option<Rc<RefCell<TreeNode>>>{
    let head = Some(Rc::new(RefCell::new(TreeNode::new(v[0].unwrap()))));
    let mut deq = std::collections::VecDeque::new();
    deq.push_back(head.as_ref().unwrap().clone());
    for (i,children) in v[1..].iter().enumerate(){
        if let Some(v) = children {
            let parent = deq.pop_front().unwrap();
            let tmp = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
            if i%2==0{
                parent.borrow_mut().left = tmp;
                deq.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            else{
                parent.borrow_mut().right = tmp;
                deq.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

#[test]
fn tree_test(){
    assert_eq!(as_tree(vec![Some(1),None,Some(2)]),Some(Rc::new(RefCell::new(TreeNode{
        val:1,
        left:None,
        right:Some(Rc::new(RefCell::new(TreeNode{
            val:2,
            left:None,
            right:None,
        })))
    }))));
}