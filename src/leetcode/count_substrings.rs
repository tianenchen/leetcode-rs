pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let cs = s.chars().collect::<Vec<char>>();
        let (mut count, len) = (0, cs.len());
        if len == 0 {
            return 0;
        }
        for i in 0..len {
            for j in 0..len - i {
                if Self::is_palindrome(&cs[j..j + i + 1]) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn count_substrings2(s: String) -> i32 {
        let cs = s.chars().collect::<Vec<char>>();
        let (mut count, len) = (0, cs.len() as i32);
        if len == 0 {
            return 0;
        }
        for i in 0..2 * len {
            let mut left = i as i32 / 2;
            let mut right = left + i as i32 % 2;
            // println!("left :{} , right :{}", left,right);
            while left >= 0 && right < len && &cs[left as usize] == &cs[right as usize] {
                count += 1;
                left -= 1;
                right += 1;
            }
        }
        count
    }

    #[inline]
    pub fn is_palindrome(subs: &[char]) -> bool {
        let len = subs.len();
        let (mut i, end) = (1, len / 2);
        for c in &subs[..end] {
            if *c != subs[len - i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

#[test]
fn check() {
    assert_eq!(Solution::is_palindrome(&['a']), true);
    assert_eq!(Solution::is_palindrome(&['a', 'a', 'a']), true);
    assert_eq!(Solution::is_palindrome(&['a', 'b', 'a']), true);
    assert_eq!(Solution::is_palindrome(&['a', 'b', 'c']), false);
    assert_eq!(Solution::is_palindrome(&['a', 'b']), false);
    assert_eq!(Solution::count_substrings2("abc".to_string()), 3);
    assert_eq!(Solution::count_substrings2("aaa".to_string()), 6);
}
