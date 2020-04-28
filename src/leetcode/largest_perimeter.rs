struct Solution;

impl Solution {
    pub fn largest_perimeter(mut a: Vec<i32>) -> i32 {
        a.sort();
        for i in (0..a.len() - 2).rev() {
            if a[i] + a[i + 1] > a[i + 2] {
                return a[i] + a[i + 1] + a[i + 2];
            }
        }
        0
    }
}
