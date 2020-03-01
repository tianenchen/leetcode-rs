struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let cs = s.chars().collect::<Vec<char>>();
        cs[n as usize..].iter().collect::<String>()+&cs[0..n as usize].iter().collect::<String>()
    }
}