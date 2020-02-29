struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1;n];
        let (mut p2,mut p3,mut p5) = (0,0,0);
        for i in 1..n{
            dp[i] = std::cmp::min(dp[p2]*2,std::cmp::min(dp[p3]*3,dp[p5]*5));
            if dp[i] == dp[p2]*2 {p2+=1}
            if dp[i] == dp[p3]*3 {p3+=1}
            if dp[i] == dp[p5]*5 {p5+=1}
        }
        dp[n-1]
    }
}

/*

输入: n = 10
输出: 12
解释: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 是前 10 个丑数。

1*1  1*2  1*3  2*2  1*5  2*3  2*4  1*3*3 1*2*5 1*3*4
*/