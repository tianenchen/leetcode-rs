struct Solution;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut res = 0;
        for i in 0..len - 1 {
            for j in i + 1..len {
                if nums[i] > nums[j] {
                    res += 1;
                }
            }
        }
        res
    }
}
