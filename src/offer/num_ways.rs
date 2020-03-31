struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        // if n == 0{1}
        // else if n < 3 {
        //     n
        // }
        // else {
        //     let res = Self::num_ways(n-1)+Self::num_ways(n-2);
        //     if res > 1000000007 {
        //         res % 1000000007
        //     }
        //     else {
        //         res
        //     }
        // }
        if n == 0 {
            return 1;
        } else if n < 3 {
            return n;
        }
        let (mut a, mut b) = (1, 2);
        let mut t;
        for _ in 3..=n {
            t = a;
            a = b;
            b += t;
            if b > 1000000007 {
                b %= 1000000007;
            }
        }
        b
    }
}

#[test]
fn check() {
    assert_eq!(Solution::num_ways(7), 21);
}
