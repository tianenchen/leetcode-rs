pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let (mut len, mut cnt, mut start, mut n) = (1_i64, 9_i64, 1_i64, n as i64);
        while n as i64 > len * cnt {
            n -= len * cnt;
            len += 1;
            cnt *= 10;
            start *= 10;
        }
        start += (n - 1) / len;
        let idx = len - (n - 1) % len;
        for _ in 1..idx {
            start /= 10;
        }
        start as i32 % 10
    }
}

#[test]
fn check() {
    assert_eq!(Solution::find_nth_digit(3), 3);
    assert_eq!(Solution::find_nth_digit(99), 4);
}

// 0-9              10 位     1 *9

// 10-19            20 位

// 10 -99           179位     20 * 9 -1

// 100-199          300 位

// 100-999          2700 位   300 * 9

// 1000-1999        4000

// 1000-9999        36000     4000 * 9
