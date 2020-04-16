struct Solution;

impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = matrix.len();
        let mut res = vec![vec![0;len]];
        let mut visited;
        //遍历每一个点
        for x in 0..len{
            for y in 0..len{
                if (matrix[y][x] == 1){
                    //初始化访问节点
                    visited = vec![vec![false];len];
                    //give up
                }
            }
        }

        vec![]
    }


    fn bfs(x:usize,y:usize,len:usize,visited:Vec<Vec<bool>>,matrix:Vec<Vec<i32>>){
        //边界内且未被访问过
        if x > 0 && x < len && y > 0 && y < len{
            if matrix[y][x] == 1{//入口节点

            }
            //遍历途中
            else if !visited[y][x]{

            }
        }
    }
}