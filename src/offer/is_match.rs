struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let p = p.chars().collect::<Vec<char>>();
        Self::help(&s, &p)
    }

    fn help(s: &[char], p: &[char]) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        let first_match = !s.is_empty() && (s[0] == p[0] || p[0] == '.'); //判断当前字符是否相等
        if p.len() > 1 && p[1] == '*' {
            Self::help(&s, &p[2..]) || (first_match && Self::help(&s[1..], &p)) //砍掉p两个字符（0个匹配）或者砍掉s一个字符
        } else {
            first_match && Self::help(&s[1..], &p[1..]) //直接匹配
        }
    }
}

#[test]
fn check() {
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
}

/*
示例 1:

输入:
s = "aa"
p = "a"
输出: false
解释: "a" 无法匹配 "aa" 整个字符串。
示例 2:

输入:
s = "aa"
p = "a*"
输出: true
解释: 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
示例 3:

输入:
s = "ab"
p = ".*"
输出: true
解释: ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
示例 4:

输入:
s = "aab"
p = "c*a*b"
输出: true
解释: 因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。
示例 5:

输入:
s = "mississippi"
p = "mis*is*p*."
输出: false

*/
