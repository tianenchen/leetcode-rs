struct Solution;

impl Solution {
    pub fn rob2(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let (mut before_sum, mut curr_sum) = (0, 0);
        for i in 0..len {
            let tmp = curr_sum;
            curr_sum = std::cmp::max(nums[i] + before_sum, curr_sum);
            before_sum = tmp;
        }
        curr_sum
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        Self::help(0, &nums)
    }
    fn help(next: usize, nums: &Vec<i32>) -> i32 {
        if next < nums.len() {
            std::cmp::max(
                nums[next] + Self::help(next + 2, nums),
                Self::help(next + 1, nums),
            )
        } else {
            0
        }
    }
}
