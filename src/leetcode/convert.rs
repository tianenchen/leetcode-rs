struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {return s}
        let mut z = vec![vec![];num_rows as usize];
        let mut cur = 0;
        let mut factor = -1;
        for c in s.chars(){
            if cur == 0 || cur == num_rows-1{
                factor *= -1;
            }
            z[cur as usize].push(c);
            cur+=factor;
        }
        z.iter().flat_map(|s| s.iter()).collect()
    }
}


#[test]
fn check() {
    assert_eq!(Solution::convert("LEETCODEISHIRING".to_string(),3),"LCIRETOESIIGEDHN".to_string());
    assert_eq!(Solution::convert("LEETCODEISHIRING".to_string(),4),"LDREOEIIECIHNTSG".to_string());
    assert_eq!(Solution::convert("AB".to_string(),1),"AB".to_string());
}