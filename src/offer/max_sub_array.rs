struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut res,mut sum) = (std::i32::MIN,0);
        for i in nums.into_iter(){
            sum = std::cmp::max(i,sum+i);
            res = std::cmp::max(res,sum);
        }
        res
    }
}