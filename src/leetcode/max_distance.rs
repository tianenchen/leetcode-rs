struct Solution;


impl Solution {
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let len = grid.len();
        let mut land = std::collections::VecDeque::new();
        let mut has_ocean = false;
        for x in 0..len{
            for y in 0..len{
                if grid[x][y] == 1{
                    land.push_back((x,y));
                }
                else{
                    has_ocean = true;
                }
            }
        }
        if !has_ocean {
            return -1
        }
        let mut last = None;
        while let Some((x,y)) = land.pop_front(){
            let tmp = grid[x][y];
            last = Some((x,y));
            if x+1 < len && grid[x+1][y] == 0 {
                grid[x+1][y] = tmp + 1;
                land.push_back((x+1,y));
            }
            if y+1 < len && grid[x][y+1] == 0{
                grid[x][y+1] = tmp + 1;
                land.push_back((x,y+1));
            }
            if x >= 1 && grid[x-1][y] == 0{
                grid[x-1][y] = tmp + 1;
                land.push_back((x-1,y));
            }
            if y >= 1 && grid[x][y-1] == 0{
                grid[x][y-1] = tmp + 1;
                land.push_back((x,y-1));
            }
        }
        if last.is_none(){
            return -1
        }
        grid[last.unwrap().0][last.unwrap().1]-1
    }
}