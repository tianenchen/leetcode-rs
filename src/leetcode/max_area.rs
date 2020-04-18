struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut slow, mut fast) = (0, height.len() - 1);
        let mut area = 0;
        while slow < fast {
            let slow_height = height[slow];
            let fast_height = height[fast];
            area = std::cmp::max(area, (fast as i32 - slow as i32) * std::cmp::min(slow_height, fast_height));
            if slow_height <= fast_height {
                slow += 1;
            } else {
                fast -= 1;
            }
        }
        area
    }
}
