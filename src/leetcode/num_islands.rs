struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let height = grid.len();
        if height == 0 {return 0}
        let width = grid[0].len();
        if width == 0 {return 0}
        let mut visited = vec![vec![false;width];height];
        let mut res = 0;
        for x in 0..width{
            for y in 0..height{
                if grid[y][x]=='1' && !visited[y][x]{
                    Self::dfs(x,y,height,width,&mut visited, &grid);
                    res+=1;
                }
            }
        }
        res
    }

    fn dfs(x:usize,y:usize,height:usize,width:usize,visited:&mut Vec<Vec<bool>>,grid:&Vec<Vec<char>>){
        //未越界且未被访问过且是陆地
        if x < width && y < height && !visited[y][x] && grid[y][x]=='1'{
            visited[y][x] = true;
            if x >= 1{
                Self::dfs(x-1,y,height, width, visited, grid);
            }
            if y >= 1{
                Self::dfs(x,y-1,height, width, visited, grid);
            }
            Self::dfs(x+1,y,height, width, visited, grid);
            Self::dfs(x,y+1,height, width, visited, grid);
        }
    }
}