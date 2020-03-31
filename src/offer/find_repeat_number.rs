struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut res = std::collections::HashSet::new();
        for i in &nums[..] {
            if res.get(i).is_some() {
                return *i;
            }
            res.insert(*i);
        }
        0
    }
}

#[test]
fn check() {
    assert_eq!(Solution::find_repeat_number(vec![0, 0, 1, 2, 3]), 0);
    assert_eq!(Solution::find_repeat_number(vec![12, 3, 4, 55, 9, 12]), 12);
    assert_eq!(Solution::find_repeat_number(vec![0, 1, 3, 3]), 3);
}
