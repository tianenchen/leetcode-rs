pub struct Solution;
impl Solution {
    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        Self::help(0, postorder.len() as i32 - 1, &postorder)
    }

    fn help(start: i32, end: i32, postorder: &Vec<i32>) -> bool {
        if start >= end {
            return true;
        }
        let start = start as usize;
        let end = end as usize;
        let root = postorder[end];
        let bound = Self::max_idx(&postorder[start..end], root) + start;
        for i in start..bound {
            if postorder[i] > root {
                return false;
            }
        }
        for i in bound..end {
            if postorder[i] < root {
                return false;
            }
        }
        Self::help(start as i32, bound as i32 - 1, &postorder)
            && Self::help(bound as i32, end as i32 - 1, &postorder)
    }

    #[inline]
    fn max_idx(v: &[i32], root: i32) -> usize {
        for (v, i) in v.iter().enumerate() {
            if *i > root {
                return v;
            }
        }
        v.len()
    }
}

#[test]
fn check() {
    // assert_eq!(Solution::max_idx(&vec![1,2],3),2);
    // assert_eq!(Solution::max_idx(&vec![],1),0);
    assert_eq!(Solution::max_idx(&vec![1, 2, 3, 4, 6], 5), 4);
    assert_eq!(Solution::verify_postorder(vec![1, 6, 3, 2, 5]), false);
    assert_eq!(Solution::verify_postorder(vec![1, 3, 2, 6, 5]), true);
    assert_eq!(Solution::verify_postorder(vec![5, 4, 3, 2, 1]), true);
    assert_eq!(
        Solution::verify_postorder(vec![4, 8, 6, 12, 16, 14, 10]),
        true
    );
}

/*
输入一个整数数组，判断该数组是不是某二叉搜索树的后序遍历结果。
如果是则返回 true，否则返回 false。假设输入的数组的任意两个数字都互不相同。


     5
    / \
   2   6
  / \
 1   3


示例 1：

输入: [1,6,3,2,5]
输出: false
示例 2：

输入: [1,3,2,6,5]
输出: true

*/
