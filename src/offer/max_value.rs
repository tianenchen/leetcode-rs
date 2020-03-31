struct Solution;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = grid[0][0];
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }
        for i in 1..n {
            dp[0][i] = dp[0][i - 1] + grid[0][i];
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = std::cmp::max(dp[i - 1][j] + grid[i][j], dp[i][j - 1] + grid[i][j]);
            }
        }
        dp[m - 1][n - 1]
    }

    fn help(sum: i32, i: usize, j: usize, i_max: usize, j_max: usize, grid: &Vec<Vec<i32>>) -> i32 {
        let v = grid[i][j];
        //上右触顶就返回
        if i == i_max && j == j_max {
            return sum + v;
        }
        if i == i_max {
            Self::help(sum + v, i, j + 1, i_max, j_max, grid)
        } else if j == j_max {
            Self::help(sum + v, i + 1, j, i_max, j_max, grid)
        } else {
            std::cmp::max(
                Self::help(sum + v, i, j + 1, i_max, j_max, grid),
                Self::help(sum + v, i + 1, j, i_max, j_max, grid),
            )
        }
    }
}
