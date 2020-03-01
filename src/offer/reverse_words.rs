struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().map(|x|x.to_string()).collect::<Vec<String>>().join(" ")
    }
}