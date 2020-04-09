struct Solution;

/*

给你两个单词 word1 和 word2，请你计算出将 word1 转换成 word2 所使用的最少操作数 。

你可以对一个单词进行如下三种操作：

插入一个字符
删除一个字符
替换一个字符


示例 1：

输入：word1 = "horse", word2 = "ros"
输出：3
解释：
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')

*/
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let s1 = word1.chars().collect::<Vec<char>>();
        let s2 = word2.chars().collect::<Vec<char>>();
        let (len1, len2) = (s1.len(), s2.len());
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        //s1的前i个转换到s2的前j个所需要的最少步骤
        for i in 0..=len1 {
            dp[i][0] = i as i32;
        }
        for j in 0..=len2 {
            dp[0][j] = j as i32;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] =
                        std::cmp::min(std::cmp::min(dp[i][j - 1], dp[i - 1][j]), dp[i - 1][j - 1])
                            + 1
                }
            }
        }
        dp[len1][len2]
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::min_distance("horse".to_string(), "ros".to_string()),
        3
    );
}
