struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut count = 0;
        for i in 0..len{
            for j in i+1..len{
                if nums[i] > 2*nums[j]{
                    count+=1;
                }
            }
        }
        count
    }
}
#[test]
fn check() {
    // println!("{}", 5&(-5));
    assert_eq!(Solution::reverse_pairs(vec![1,3,2,3,1]),2);
    assert_eq!(Solution::reverse_pairs(vec![2,4,3,5,1]),3);

}