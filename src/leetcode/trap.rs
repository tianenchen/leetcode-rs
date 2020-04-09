struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let len = height.len();
        let mut left_max = vec![0; len];
        let mut right_max = vec![0; len];
        left_max[0] = height[0];
        right_max[len - 1] = height[len - 1];
        for i in 1..len {
            left_max[i] = std::cmp::max(height[i], left_max[i - 1]);
        }
        for i in (0..len - 1).rev() {
            right_max[i] = std::cmp::max(height[i], right_max[i + 1]);
        }
        let mut res = 0;
        for i in 0..len {
            res += std::cmp::min(left_max[i], right_max[i]) - height[i];
        }
        res
    }
}
