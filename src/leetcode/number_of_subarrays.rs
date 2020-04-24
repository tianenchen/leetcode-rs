struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut count = 0;
        let (mut left, mut right) = (0, 0);
        let mut res = 0;
        loop {
            // dbg!(left, right, count, res);
            if count < k && right < len {
                if nums[right] % 2 == 1 {
                    count += 1;
                }
                if count == k {
                    res += 1;
                }
                right += 1;
            } else if left < right {
                if nums[left] % 2 == 1 {
                    count -= 1;
                }
                if count == k {
                    res += 1;
                }
                left += 1;
            } else {
                break;
            }
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    // assert_eq!(Solution::number_of_subarrays(vec![2,2,2,1,2,2,1,2,2,2], 2), 16);
}
