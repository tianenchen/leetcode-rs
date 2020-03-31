struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let end = Self::binary_search(&nums, target);
        let mut set = std::collections::HashSet::new();
        for i in (0..=end as i32).rev() {
            let val = nums[i as usize];
            if set.contains(&(target - val)) {
                return vec![val, target - val];
            } else {
                set.insert(val);
            }
        }
        vec![]
    }

    #[inline]
    fn binary_search(nums: &Vec<i32>, max: i32) -> usize {
        let (mut start, mut end) = (0, nums.len() - 1);
        while start < end {
            let mid = (start + end) >> 1;
            if nums[mid] < max {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        start
    }
}

#[test]
fn check() {
    assert_eq!(Solution::binary_search(&vec![1, 2, 3, 4, 5, 6, 7, 8], 4), 3);
}
