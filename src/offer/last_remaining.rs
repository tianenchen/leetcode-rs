struct Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        let mut res = 0;
        for i in 2..=n as i32 {
            res = (res + m) % i;
        }
        res
    }
}
