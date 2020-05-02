struct Solution;

impl Solution {
    #[allow(irrefutable_let_patterns)]
    pub fn is_happy(mut n: i32) -> bool {
        let mut s = std::collections::HashSet::new();
        let mut next = 0;
        while let c = n % 10 {
            if n == 0 {
                if s.contains(&next) {
                    return false;
                }
                if next == 1 {
                    return true;
                }
                n = next;
                // dbg!(next);
                s.insert(next);
                next = 0;
            } else {
                next += c * c;
                n /= 10;
            }
        }
        true
    }
}

#[test]
fn check() {
    assert_eq!(Solution::is_happy(19), true);
    assert_eq!(Solution::is_happy(18), false);
    assert_eq!(Solution::is_happy(1), true);
    assert_eq!(Solution::is_happy(7), true);
}
