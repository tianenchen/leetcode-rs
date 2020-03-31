struct Solution;
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let dict = chars.chars().collect::<Vec<char>>();
        let mut res = 0;
        'outer: for s in words {
            let mut tmp = dict.clone();
            let mut count = 0;
            for c in s.chars() {
                let pos = tmp.iter().position(|x| *x == c);
                if let Some(pos) = pos {
                    tmp.remove(pos);
                    count += 1;
                } else {
                    continue 'outer;
                }
            }
            res += count;
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::count_characters(
            vec![
                "cat".to_string(),
                "bt".to_string(),
                "hat".to_string(),
                "tree".to_string()
            ],
            "atach".to_string()
        ),
        6
    );
    assert_eq!(
        Solution::count_characters(
            vec![
                "hello".to_string(),
                "world".to_string(),
                "leetcode".to_string()
            ],
            "welldonehoneyr".to_string()
        ),
        10
    );
}
