struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let mut visited = vec![vec![false; image[0].len()]; image.len()];
        Self::dfs(sc, sr, new_color, image[sr][sc], &mut visited, &mut image);
        image
    }

    fn dfs(
        x: usize,
        y: usize,
        new_color: i32,
        origin_color: i32,
        visited: &mut Vec<Vec<bool>>,
        turned: &mut Vec<Vec<i32>>,
    ) {
        //界内
        if y < turned.len() && x < turned[0].len() && !visited[y][x] && turned[y][x] == origin_color
        {
            turned[y][x] = new_color;
            visited[y][x] = true;
            if x > 0 {
                Self::dfs(x - 1, y, new_color, origin_color, visited, turned);
            }
            if y > 0 {
                Self::dfs(x, y - 1, new_color, origin_color, visited, turned);
            }
            Self::dfs(x + 1, y, new_color, origin_color, visited, turned);
            Self::dfs(x, y + 1, new_color, origin_color, visited, turned);
        }
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
    );
    assert_eq!(
        Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1),
        vec![vec![0, 0, 0], vec![0, 1, 1]]
    );
    assert_eq!(
        Solution::flood_fill(vec![vec![0, 0, 0], vec![1, 0, 0]], 1, 0, 2),
        vec![vec![0, 0, 0], vec![2, 0, 0]]
    );
}
