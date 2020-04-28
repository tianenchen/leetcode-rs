struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut start, mut end) = (0, nums.len() - 1);
        while start <= end {
            let mid = start + end >> 1;
            if nums[mid] == target {
                return mid as i32;
            }
            //说明0=》mid是递增的
            if nums[mid] >= nums[start] {
                if nums[mid] > target && target >= nums[start] {
                    if mid > 0 {
                        end = mid - 1;
                    } else {
                        return -1;
                    }
                }
                //确定值位于mid右侧
                else {
                    start = mid + 1;
                }
            }
            //说明mid=》end是递增的
            else {
                if nums[mid] < target && target <= nums[end] {
                    start = mid;
                } else {
                    if mid > 0 {
                        end = mid - 1;
                    } else {
                        return -1;
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn check() {
    assert_eq!(2 + 3 >> 1, 2);
    assert_eq!(Solution::search(vec![1], 1), 0);
    assert_eq!(Solution::search(vec![1, 2], 2), 1);
    assert_eq!(Solution::search(vec![2, 1], 1), 1);
    assert_eq!(Solution::search(vec![2, 1], 2), 0);
    assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
}
