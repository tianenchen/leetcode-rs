struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut count = 0;
        nums.iter().for_each(|i| {
            if *i == target {
                count += 1;
            }
        });
        count
    }

    fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let len = nums.len();
        let (mut left, mut right) = (0, len - 1);
        while left < right {
            let mid = left + right >> 1;
            if target > nums[mid] {
                left = mid + 1
            } else {
                right = mid
            }
        }
        if nums[left] != target {
            return 0;
        }
        let tmp = left;
        right = len - 1;
        while left < right {
            let mid = left + right + 1 >> 1;
            if nums[mid] != target {
                right = mid - 1
            } else {
                left = mid
            }
        }
        (right - tmp + 1) as i32
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::binary_search(vec![1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7], 7),
        Solution::search(vec![1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7], 7)
    );
}
