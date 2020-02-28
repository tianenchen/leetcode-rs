struct Solution;
impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        let cs = num.to_string().chars().collect::<Vec<char>>();
        let mut dp = vec![1;cs.len()+1];
        for i in 2..=cs.len(){
            if cs[i-2] == '1' || (cs[i-2]=='2' && cs[i-1] < '6'){
                dp[i] = dp[i-2] + dp[i-1];
            }
            else{
                dp[i] = dp[i-1];
            }
        }
        dp[cs.len()]
    }
}