use crate::util::tree::*;

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if !nums.is_empty() {
            let mid = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: Self::sorted_array_to_bst((&nums[0..mid]).to_vec()),
                right: Self::sorted_array_to_bst((&nums[mid + 1..]).to_vec()),
            })))
        } else {
            None
        }
    }

    // fn help(nums: &[i32])->Option<Rc<RefCell<TreeNode>>>{
    //     if !nums.is_empty(){
    //         let val = nums[nums.len()/2];
    //         let mid = nums.len()/2;
    //         Some(Rc::new(RefCell::new(TreeNode{
    //             val:nums[mid],
    //             left:Self::help(&nums[0..mid]),
    //             right:Self::help(&nums[mid+1..]),
    //         })))
    //     }
    //     else{
    //         None
    //     }
    // }
}
