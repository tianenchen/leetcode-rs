struct Solution;

impl Solution {
    pub fn compress_string(mut s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let len = s.len();
        s.push('0');
        let mut cs = s.chars();
        let mut tmp = cs.next().unwrap();
        let mut i = 1;
        let mut res = String::new();
        for c in cs {
            if tmp == c {
                i += 1;
            } else {
                res.push(tmp);
                res += &i.to_string();
                i = 1;
                tmp = c;
            }
        }
        if len > res.len() {
            res
        } else {
            s.remove(len);
            s
        }
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::compress_string("aaabbcdde".to_string()),
        "aaabbcdde".to_string()
    );
}
