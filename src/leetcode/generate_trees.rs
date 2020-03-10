use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
		// let mut res = vec![];
		let mut dp :Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>= vec![vec![];n as usize+1];
		dp[0].push(None);
		for i in 1..=n as usize{
			for j in 1..=i{
				let left = dp[j-1].clone();
				let right = dp[i-j].clone();
				let v = &mut dp[i];
				for l in &left{
					for r in &right{
						let mut root = Some(Rc::new(RefCell::new(TreeNode::new(j as i32))));
						root.as_mut().unwrap().borrow_mut().left = Self::add(&l.clone(), 0);
						root.as_mut().unwrap().borrow_mut().right = Self::add(&r.clone(), j as i32);
						v.push(root);
					}
				}
			}
		}
		dp[n as usize].clone()
	}
	
	fn add(root :&Option<Rc<RefCell<TreeNode>>>,v:i32)->Option<Rc<RefCell<TreeNode>>>{
		if let Some(root) = root {
			let mut node = Some(Rc::new(RefCell::new(TreeNode::new(root.borrow().val+v))));
			node.as_mut().unwrap().borrow_mut().left = Self::add(&root.borrow().left, v);
			node.as_mut().unwrap().borrow_mut().right = Self::add(&root.borrow().right, v);
			return node
		}
		None
	}

    fn help(left:i32,right:i32,res:&mut Vec<Option<Rc<RefCell<TreeNode>>>>)->Rc<RefCell<TreeNode>>{
		for v in left..=right{
			let mut root = Rc::new(RefCell::new(TreeNode::new(v)));
			let left = Self::help(left,v-1,res);
			let right = Self::help(v+1,right,res);
			res.push(Some(root));
		}


		unimplemented!()
    }
}

#[cfg(test)]
mod tests{
	use crate::util::tree::*;
	use super::*;
	#[test]
	fn check() {
		assert_eq!(vec![as_tree(vec![Some(1),None,Some(2)]),as_tree(vec![Some(2),Some(1)])],Solution::generate_trees(2));
	}
}