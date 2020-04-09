struct Solution;

impl Solution {
    pub fn rotate2(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        let l = matrix.len();
        for i in 0..l / 2 {
            for j in 0..l {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[l - 1 - i][j];
                matrix[l - 1 - i][j] = tmp;
            }
        }
        for i in 0..l {
            for j in 0..l {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }

    //4bit
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        let l = matrix.len();
        for i in 0..l {
            for j in 0..l {
                matrix[i][l - 1 - j] = (matrix[i][l - 1 - j] & 15) | ((matrix[j][i] & 15) << 4);
            }
        }
        for i in 0..l {
            for j in 0..l {
                matrix[i][j] = matrix[i][j] >> 4;
            }
        }
    }
}

#[test]
fn check() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
}
