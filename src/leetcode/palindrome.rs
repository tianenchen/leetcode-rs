struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cs: Vec<char> = s.chars().collect::<Vec<char>>();
        if cs.is_empty() {
            return true;
        }
        let (mut head, mut tail) = (0, cs.len() - 1);
        loop {
            if head == tail || tail + 1 == head {
                return true;
            }
            if !cs[head].is_alphanumeric() {
                head += 1;
                continue;
            }
            if !cs[tail].is_alphanumeric() {
                tail -= 1;
                continue;
            }
            if cs[head].eq_ignore_ascii_case(&cs[tail]) {
                head += 1;
                tail -= 1;
            } else {
                return false;
            }
        }
    }
}

#[test]
fn check() {
    assert_eq!(Solution::is_palindrome("abcba".to_string()), true);
    // assert_eq!(Solution::is_palindrome("a b c b a".to_string()),true);
    assert_eq!(Solution::is_palindrome("dd".to_string()), true);
    // assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),true);
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
}
