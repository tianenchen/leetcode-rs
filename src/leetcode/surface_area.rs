pub struct Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let len = grid.len();
        let mut area = 0;
        for x in 0..len {
            for y in 0..len {
                if grid[x][y] > 0 {
                    let tmp = 6 * grid[x][y] - 2 * (grid[x][y] - 1) as i32;
                    let mut overlap_right = 0;
                    if x + 1 < len {
                        overlap_right = std::cmp::min(grid[x + 1][y], grid[x][y]) * 2;
                    }
                    let mut overlap_up = 0;
                    if y + 1 < len {
                        overlap_up = std::cmp::min(grid[x][y + 1], grid[x][y]) * 2;
                    }
                    area += tmp - overlap_right - overlap_up;
                }
            }
        }
        area
    }
}

#[test]
fn check() {
    assert_eq!(Solution::surface_area(vec![vec![1, 1], vec![1, 1]]), 16);
}
