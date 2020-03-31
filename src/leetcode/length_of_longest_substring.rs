struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut h, mut t, mut res, mut set, len) =
            (0, 0, 0, std::collections::HashSet::new(), s.len());
        let s = s.chars().collect::<Vec<char>>();
        while h < len && t < len {
            if set.contains(&s[t]) {
                set.remove(&s[h]);
                h += 1;
            } else {
                set.insert(s[t]);
                t += 1;
                if t - h > res {
                    res = t - h;
                }
            }
        }
        res as i32
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::length_of_longest_substring("abcdeffg".to_string()),
        6
    );
}
