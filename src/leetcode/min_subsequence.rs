struct Solution;

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut half :i32 = nums.iter().sum::<i32>() >> 1;
        let mut res = vec![];
        for c in nums.into_iter().rev(){
            half-=c;
            res.push(c);
            if half < 0 {
                return res;
            }
        }
        vec![]
    }
}