struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome2(s: String) -> i32 {
        let cs = s.chars().collect::<Vec<char>>();
        if cs.is_empty(){
            return 0
        }
        let mut set = std::collections::HashSet::new();
        let mut res = 0;

        for c in cs{
            if set.contains(&c){
                set.remove(&c);
                res+=2;
            }
            else{
                set.insert(c);
            }
        }
        if set.is_empty(){
            res
        }
        else{
            res+1
        }
    }

    pub fn longest_palindrome(s: String) -> String {
        let cs = s.chars().collect::<Vec<char>>();
        if cs.is_empty(){
            return String::default()
        }
        let len = cs.len();
        let mut max = 0;
        let mut res = cs[0].to_string();
        for (i,c) in cs.iter().enumerate(){
            let mut tmp = 0;
            for j in 1..std::cmp::min(len-i,i+1){
                if cs[i+j]!=cs[i-j]{
                    break
                }
                tmp+=1;
            }
            if tmp > max {
                res = cs[i-tmp..=i+tmp].iter().collect();
                max = tmp;
            }
            if i+1<len{
                if *c==cs[i+1]{
                    tmp = 0;
                    for j in 1..std::cmp::min(len-i-1,i+1){
                        if cs[i+1+j]!=cs[i-j]{
                            break
                        }
                        tmp+=1;
                    }
                    if tmp >= max{
                        res = cs[i-tmp..=i+1+tmp].iter().collect();
                        max = tmp;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn check() {

    assert_eq!(Solution::longest_palindrome2("abccccdd".to_string()),7);





    assert_eq!(Solution::longest_palindrome("aba".to_string()),"aba".to_string());
    assert_eq!(Solution::longest_palindrome("a".to_string()),"a".to_string());
    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()),"cccc".to_string());
    assert_eq!(Solution::longest_palindrome("abcccccdd".to_string()),"ccccc".to_string());
    assert_eq!(Solution::longest_palindrome("aa".to_string()),"aa".to_string());
}