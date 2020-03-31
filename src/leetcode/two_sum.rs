struct Solution;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, var) in nums.into_iter().enumerate() {
            if let Some(another_index) = map.get(&var) {
                return vec![*another_index, i as i32];
            }
            map.insert(target - var, i as i32);
        }
        vec![]
    }
}

#[test]
fn check() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 4, 3], 9), vec![0, 1]);
}
