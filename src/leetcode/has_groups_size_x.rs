struct Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut v = vec![0; 10000];
        for i in &deck {
            v[*i as usize] += 1;
        }
        let mut gcd = -1;
        for i in &deck {
            if gcd == -1 {
                gcd = v[*i as usize];
            } else {
                gcd = Self::gcd(v[*i as usize], gcd);
            }
        }
        gcd >= 2
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a % b == 0 {
            b
        } else {
            Self::gcd(b, a % b)
        }
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3, 3]),
        true
    );
    assert_eq!(
        Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]),
        false
    );
    assert_eq!(Solution::has_groups_size_x(vec![1, 1]), true);
    assert_eq!(
        Solution::has_groups_size_x(vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2]),
        true
    );
}
