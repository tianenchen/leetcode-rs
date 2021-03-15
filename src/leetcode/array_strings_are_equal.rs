struct Solution;


impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.into_iter().collect::<String>()==word2.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert!(Solution::array_strings_are_equal(vec![], vec![]));
        assert!(Solution::array_strings_are_equal(vec!["a".to_owned(),"b".to_owned()], vec!["ab".to_owned()]))
    }
}