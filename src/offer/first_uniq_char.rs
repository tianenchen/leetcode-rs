struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        let mut dict = vec![(' ',0);26];
        for c in s.chars(){
            let idx = c as usize - 97;
            dict[idx] = (c,dict[idx].1+1)
        }
        for c in s.chars(){
            if dict[c as usize - 97].1 == 1{
                return c
            }
        }
        ' '
    }
}


#[test]
fn check() {
    assert_eq!('a' as u8 ^ 'b' as u8 ^ 'a' as u8,'b' as u8);
}