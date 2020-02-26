pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty(){
            return vec![];
        }
        let (mut h_max,mut h_min,mut w_max,mut w_min) = (matrix.len()-1,0,matrix[0].len()-1,0);
        let mut res = vec![];
        let (mut x,mut y) = (0,0);
        while h_max>=h_min&&w_max>=w_min{
            if y == h_min && x == w_min{ //左下角
                for i in w_min..=w_max{
                    res.push(matrix[y][i]);
                }
                x=w_max;
                y+=1;
                h_min+=1;
            }
            else if x == w_max && y == h_min{//peak 右下角
                for i in h_min..=h_max{
                    res.push(matrix[i][x]);
                }
                y=h_max;
                x-=1;
                w_max-=1;
            }
            else if y == h_max && x == w_max{ //右上角
                for i in (w_min..=w_max).rev(){
                    res.push(matrix[y][i]);
                }
                x=w_min;
                y-=1;
                h_max-=1;
            }
            else if x == w_min && y == h_max{ //左上角
                for i in (h_min..=h_max).rev(){
                    res.push(matrix[i][x]);
                }
                y=h_min;
                x+=1;
                w_min+=1;
            }
        }
        res
    }
}


#[test]
fn check() {
    assert_eq!(Solution::spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]]),vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    assert_eq!(Solution::spiral_order(vec![vec![1]]),vec![1]);
    // assert_eq!(Solution::spiral_order(vec![vec![]]),vec![]);
    assert_eq!(Solution::spiral_order(vec![vec![3],vec![2]]),vec![3,2]);
}


//输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
//输出：[1,2,3,4,8,12,11,10,9,5,6,7]

// [0][0] [0][1] [0][2] [0][3] [1][3] [2][3] [3][3] [3][2] [3][1] [3][0] [2][0] [1][0] [1][1]

//  o = 3 n = 3                