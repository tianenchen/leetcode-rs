pub struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let (max_step, width) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; width]; max_step]; //只要visited都跳过
        let mut max_res = 0;
        for y in 0..max_step {
            for x in 0..width {
                let mut res = 0;
                Self::help(x, y, width, max_step, &mut visited, &mut res, &grid);
                max_res = std::cmp::max(max_res, res);
            }
        }
        max_res
    }

    fn help(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        visited: &mut Vec<Vec<bool>>,
        res: &mut i32,
        grid: &Vec<Vec<i32>>,
    ) {
        if x < width && y < height && !visited[y][x] {
            //界内
            visited[y][x] = true;
            if grid[y][x] == 1 {
                *res += 1;
                Self::help(x + 1, y, width, height, visited, res, grid);
                if x > 0 {
                    Self::help(x - 1, y, width, height, visited, res, grid);
                }
                Self::help(x, y + 1, width, height, visited, res, grid);
                if y > 0 {
                    Self::help(x, y - 1, width, height, visited, res, grid);
                }
            }
        }
    }
}
