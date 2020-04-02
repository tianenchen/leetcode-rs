struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut depth = 0;
        let mut res = vec![];
        for c in seq.chars() {
            if c == '(' {
                depth += 1;
                res.push(depth % 2);
            } else {
                res.push(depth % 2);
                depth -= 1;
            }
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::max_depth_after_split("()(())()".to_string()),
        vec![1, 1, 1, 0, 0, 1, 1, 1]
    );
}
