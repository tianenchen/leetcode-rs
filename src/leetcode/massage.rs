struct Solution;

impl Solution {
    pub fn massage(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let len = nums.len();
        let mut dp = vec![(0, 0); len];
        dp[0] = (nums[0], 0); //(选了,没选)
        for i in 1..len {
            dp[i] = (
                dp[i - 1].1 + nums[i],
                std::cmp::max(dp[i - 1].0, dp[i - 1].1),
            );
        }
        std::cmp::max(dp[len - 1].0, dp[len - 1].1)
    }

    pub fn massage1(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        Self::help(0, &nums)
    }

    fn help(idx: usize, nums: &Vec<i32>) -> i32 {
        if idx < nums.len() - 1 {
            std::cmp::max(
                nums[idx] + Self::help(idx + 2, &nums),
                Self::help(idx + 1, nums),
            )
        } else if idx == nums.len() - 1 {
            nums[idx]
        } else {
            0
        }
    }
}
//[0,0,2,1,4,5,3,1,1,3]

#[test]
fn check() {
    assert_eq!(Solution::massage(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::massage(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(Solution::massage(vec![2, 1, 4, 5, 3, 1, 1, 3]), 12);
}
