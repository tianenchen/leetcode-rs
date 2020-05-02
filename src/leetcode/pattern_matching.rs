struct Solution;
impl Solution {
    pub fn pattern_matching(pattern: String, value: String) -> bool {
        let (mut counta, mut countb) = (0, 0);
        let value = value.chars().collect::<Vec<char>>();
        let mut first = 'a';
        for (i, c) in pattern.chars().enumerate() {
            if i == 0 {
                first = c;
            }
            if c == 'a' {
                counta += 1;
            } else {
                countb += 1;
            }
        }
        let mut a_words = 0;
        let len = value.len();
        loop {
            let b_words = (len - counta * a_words) / countb;
            a_words += 1;
            //give up
        }
        false
    }
}
