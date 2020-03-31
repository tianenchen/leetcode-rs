struct Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let x = Self::last_remaining(n-1,m);
        (x%n+m)%n
    }
}