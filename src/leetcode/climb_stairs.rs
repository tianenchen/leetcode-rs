struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 0 {
            1
        } else if n < 3 {
            n
        } else {
            Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2)
        }
    }

    fn climb_stairs2(n: i32) -> i32 {
        if n == 0 {
            return 1;
        } else if n < 3 {
            return n;
        }
        let mut a = 1;
        let mut b = 2;
        let mut res = 0;
        for _ in 3..=n {
            res = a + b;
            a = b;
            b = res;
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(Solution::climb_stairs(33), Solution::climb_stairs2(33));
}
