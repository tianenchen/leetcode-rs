struct Solution;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty(){ return 0 }
        let mut dp = vec![0;nums.len()];
        dp[0] = 1;
        let mut res = 1;
        for i in 1..nums.len(){
            let mut max = 0;
            for j in 0..i{
                if nums[i] > nums[j] {
                    max = std::cmp::max(max,dp[j]);//求之前每一次的最大值
                }
            }
            dp[i] = max+1;//如果有比他小的则加一就是当前值
            res = std::cmp::max(res,dp[i]);
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]),4);
}

/*
输入: [10,9,2,5,3,7,101,18]
输出: 4 
解释: 最长的上升子序列是 [2,3,7,101]，它的长度是 4。
*/