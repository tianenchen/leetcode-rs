struct Solution;

impl Solution {
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let (mut start, mut end, mut max, mut c) = (0, 0, 0, 0);
        while end < a.len() {
            if a[end] == 1 {
                c += 1;
            } else if k > 0 {
                k -= 1;
                c += 1;
            } else {
                while a[start] != 0 && start < end {
                    start += 1;
                    c -= 1;
                }
                if a[start] == 0 {
                    c = end - start;
                    start += 1;
                }
            }
            end += 1;
            max = std::cmp::max(max, c);
        }
        max as i32
    }
}
