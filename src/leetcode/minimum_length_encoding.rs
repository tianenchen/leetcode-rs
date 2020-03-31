struct Solution;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut set = words
            .clone()
            .into_iter()
            .collect::<std::collections::HashSet<String>>();
        for s in words {
            let cs = s.chars().collect::<Vec<_>>();
            let len = cs.len();
            for i in 1..len {
                set.remove(&cs[i..len].iter().collect::<String>());
            }
        }
        let mut res = 0;
        for i in set {
            res += i.len() + 1;
        }
        res as i32
    }
}
