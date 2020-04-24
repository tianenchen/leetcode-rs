struct Solution;

impl Solution {
    pub fn ways_to_change(n: i32) -> i32 {
        let coins = vec![1, 5, 10, 25];
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        for coin in coins {
            for i in coin..=n {
                dp[i as usize] += dp[(i - coin) as usize];
                dp[i as usize] %= 1000000007;
            }
        }
        dp[n as usize]
    }
}
