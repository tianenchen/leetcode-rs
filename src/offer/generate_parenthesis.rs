struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let n = n as usize;
        Self::help(n, n, "".to_string(), &mut res);
        res
    }

    fn help(left: usize, right: usize, next: String, res: &mut Vec<String>) {
        if left == 0 && right == 0 {
            res.push(next.to_string());
            return;
        }
        if left > 0 {
            Self::help(left - 1, right, next.to_string() + "(", res);
        }
        if right > left {
            Self::help(left, right - 1, next.to_string() + ")", res);
        }
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    );
}
