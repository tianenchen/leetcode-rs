struct Solution;


impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut max ,mut c) = (0,0);
        for n in nums{
            if n == 1 {
                c+=1;
            }
            else{
                c=0;
            }
            max = std::cmp::max(max,c);
        }
        max
    }
}


#[test]
fn check() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1]),3);

    assert_eq!(Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1,1,1,1,1,1]),8);

    assert_eq!(Solution::find_max_consecutive_ones(vec![0,0,0,0,0,0]),0);

}