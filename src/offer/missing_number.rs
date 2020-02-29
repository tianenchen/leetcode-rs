struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut c = 0;
        for i in nums{
            if i != c {
                break
            }
            c+=1;
        }
        c
    }
}


#[test]
fn check() {
    assert_eq!(Solution::missing_number(vec![0,1,2,3,4,5,7]),6);
}