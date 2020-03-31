struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n {
            0 => 1.0,
            1 => x, // 最终都会变成 x*x  or  1/x * 1/x
            -1 => 1.0 / x,
            _ => Self::my_pow(x, n / 2) * Self::my_pow(x, n / 2) * Self::my_pow(x, n % 2),
        }
    }
}

#[test]
fn check() {
    assert_eq!(Solution::my_pow(2_f64, 4), 16_f64);
}
