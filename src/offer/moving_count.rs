struct Solution;
// #[allow(unused_imports)]
// use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut visited = vec![];
        for _ in 0..m{
            let mut line = vec![];
            for _ in 0..n{
                line.push(false);
            }
            visited.push(line);
        }
        let mut step = 0;
        Self::dfs(m,n,k,0,0,&mut step,&mut visited);
        step
    }

    fn dfs(m:i32,n:i32,k:i32,x:i32,y:i32,step:&mut i32,visited:&mut Vec<Vec<bool>>){
        if x<0 || x>=n || y<0 || y>= m || Self::sum(x)+Self::sum(y) > k || visited[y as usize][x as usize] {return}
        *step+=1;
        visited[y as usize][x as usize] = true;
        Self::dfs(m,n,k,x+1,y,step,visited);
        Self::dfs(m,n,k,x,y+1,step,visited);
        Self::dfs(m,n,k,x-1,y,step,visited);
        Self::dfs(m,n,k,x,y-1,step,visited);
    }

    fn sum(mut i:i32)->i32{
        let mut res = 0;
        while i>0{
            res += i%10;
            i/=10;
        }
        res
    }
}

#[test]
fn check() {
    // Solution::moving_count(2,3,1);
    assert_eq!(Solution::moving_count(2,3,1),3);
}