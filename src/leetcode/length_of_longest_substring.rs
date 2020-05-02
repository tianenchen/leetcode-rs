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

    pub fn length_of_longest_substring2(s: String) -> i32 {
        let (mut head, mut tail) = (0, 0);
        let mut set = std::collections::HashSet::new();
        let cs = s.chars().collect::<Vec<char>>();
        let mut max = 0;
        while tail < cs.len() {
            while set.contains(&cs[tail]) {
                set.remove(&cs[head]);
                head += 1;
            }
            set.insert(cs[tail]);
            tail += 1;
            max = std::cmp::max(set.len(), max);
        }
        max as i32
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::length_of_longest_substring2("abcdeffg".to_string()),
        6
    );
}
