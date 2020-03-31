struct Solution;

impl Solution {
    pub fn is_straight(mut nums: Vec<i32>) -> bool {
        let mut zero = 0;
        Self::quick_sort_rev(0, nums.len() - 1, &mut nums);
        if nums.is_empty() {
            return false;
        }
        let mut before = nums[0];
        for i in &nums[1..] {
            if *i == 0 {
                zero += 1;
                continue;
            }
            if before == *i {
                return false;
            }
            zero -= before - i - 1;
            before = *i;
        }
        zero >= 0
    }

    fn quick_sort_rev(left: usize, right: usize, nums: &mut Vec<i32>) {
        if left < right {
            let base = nums[left];
            let (mut i, mut j) = (left, right);
            while i != j {
                while i < j && base >= nums[j] {
                    j -= 1;
                }
                nums[i] = nums[j];
                while i < j && nums[i] > base {
                    i += 1;
                }
                nums[j] = nums[i];
            }
            nums[i] = base;
            if i > 0 {
                Self::quick_sort_rev(left, i - 1, nums)
            }
            Self::quick_sort_rev(i + 1, right, nums);
        }
    }
}

#[test]
fn check() {
    let mut v = vec![1, 3, 4, 5, 2, 0];
    Solution::quick_sort_rev(0, 5, &mut v);
    assert_eq!(v, vec![5, 4, 3, 2, 1, 0]);
}
