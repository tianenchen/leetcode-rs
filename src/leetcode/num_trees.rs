struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp  = vec![0;n+1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n{
            for j in 1..=i{
                dp[i] += dp[j-1]*dp[i-j];
            }
        }
        dp[n]
        // Self::help(1,n)
    }

    fn help(start:i32,end:i32) -> i32{
        if start < end {
            let mut sum = 0;
            for i in start..=end{
                sum += Self::help(i+1,end)*Self::help(start,i-1);
            }
            sum
        }
        else{
            1
        }
    }
}

#[test]
fn check() {
    assert_eq!(Solution::num_trees(3),5);
}