struct Solution;


impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize - 1;
        if nums.is_empty() { return vec![] }
        let mut max = Self::max(&nums[..=k]);
        let mut res = vec![];
        for i in k..nums.len(){
            if i-k > 0 && nums[i-k-1] == max{
                max = Self::max(&nums[i-k..=i]);
            }
            else {
                max = std::cmp::max(max,nums[i]);
            }
            res.push(max);
        }
        res
    }

    #[inline]
    fn max(nums :&[i32]) -> i32{
        let mut max = std::i32::MIN;
        for i in nums{
            max = std::cmp::max(max,*i);
        }
        max
    }
}