struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack = std::collections::LinkedList::new();
        let mut res = std::collections::LinkedList::new();
        for c in s.chars() {
            if !stack.is_empty() {
                res.push_back(c);
            }
            if c == '(' {
                stack.push_back(c);
            } else {
                if *stack.back().unwrap() == '(' {
                    stack.pop_back();
                } else {
                    stack.push_back(c);
                }
            }
            if stack.is_empty() {
                res.pop_back();
            }
        }
        res.into_iter().collect::<String>()
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())".to_string()),
        "()()()".to_string()
    );
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
        "()()()()(())".to_string()
    );
    assert_eq!(
        Solution::remove_outer_parentheses("()()".to_string()),
        "".to_string()
    );
}
