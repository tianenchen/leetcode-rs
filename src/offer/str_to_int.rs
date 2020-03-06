pub struct Solution;

impl Solution {
    pub fn str_to_int(str: String) -> i32 {
        let mut s = str.split_whitespace();
        if let Some(s) = s.next() {
            let mut cs = s.chars();
            let mut sign = 1;
            let mut first = 0;
            if let Some(start) = cs.next() {
                if start.is_ascii_digit(){
                    first = start.to_digit(10).unwrap();
                }
                else if start == '-'{
                    sign = -1;
                }
                else if start == '+'{

                }
                else{
                    return 0;
                }
            }
            for c in cs{
                if let Some(n) = c.to_digit(10) {
                    if first > 214748364 || (first == 214748364 && n >7) {
                        if sign > 0{
                            return std::i32::MAX
                        }
                        else {
                            return std::i32::MIN
                        }
                    }
                    first = first*10 + n;
                }
                else {
                    break;
                }
            }
            sign * first as i32
        }
        else{
            0
        }
    }
}


#[test]
fn check() {
    assert_eq!(Solution::str_to_int("   -41".to_string()),-41);
    assert_eq!(Solution::str_to_int("   -41  12".to_string()),-41);
    assert_eq!(Solution::str_to_int("a   -41  12".to_string()),0);
    assert_eq!(Solution::str_to_int("-91283472332".to_string()),std::i32::MIN);
    assert_eq!(Solution::str_to_int("2147483648".to_string()),std::i32::MAX);


    // let s = "a b c";

    // let mut cs = s.chars();
    // cs.next();

    // for i in cs{
    //     println!("{}", i);
    // }
}