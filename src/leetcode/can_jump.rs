struct Solution;
impl Solution {
    pub fn can_jump2(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut rightmost = 0;
        for i in 0..len {
            if i > rightmost {
                return false;
            }
            rightmost = std::cmp::max(rightmost, i + nums[i] as usize);
        }
        true
    }

    pub fn can_jump(nums: Vec<i32>) -> bool {
        Self::help(0, &nums)
    }

    fn help(cur: usize, nums: &Vec<i32>) -> bool {
        //越界，不会发生
        if cur >= nums.len() {
            return true;
        }
        if nums[cur] == 0 {
            return cur == nums.len() - 1;
        }
        for i in 1..=nums[cur] as usize {
            if Self::help(cur + i, nums) {
                return true;
            }
        }
        false
    }
}
