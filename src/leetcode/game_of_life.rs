struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let neighbors = vec![0, -1, 1];
        let m = board.len();
        let n = board[0].len();
        for x in 0..m {
            for y in 0..n {
                let mut lives = 0;
                'outer: for i in 0..3_i32 {
                    for j in 0..3_i32 {
                        if !(neighbors[i as usize] == 0 && neighbors[j as usize] == 0) {
                            let new_x = x as i32 + neighbors[i as usize];
                            let new_y = y as i32 + neighbors[j as usize];
                            if new_x >= 0
                                && new_x < m as i32
                                && new_y >= 0
                                && new_y < n as i32
                                && board[new_x as usize][new_y as usize] & 1 == 1
                            {
                                lives += 1;
                            }
                            if lives > 3 {
                                break 'outer;
                            }
                        }
                    }
                }
                //如果活细胞周围八个位置的活细胞数少于两个，则该位置活细胞死亡；
                //如果活细胞周围八个位置有超过三个活细胞，则该位置活细胞死亡；
                if board[x][y] & 1 == 1 && (lives < 2 || lives > 3) {
                    board[x][y] = 1; //01
                }
                //如果死细胞周围正好有三个活细胞，则该位置死细胞复活；
                if board[x][y] & 1 == 0 && lives == 3 {
                    board[x][y] = 2; //10
                }
                //如果活细胞周围八个位置有两个或三个活细胞，则该位置活细胞仍然存活；
                if board[x][y] & 1 == 1 && (lives == 2 || lives == 3) {
                    board[x][y] = 3; //11
                }
            }
        }
        for x in 0..m {
            for y in 0..n {
                board[x][y] = board[x][y] >> 1;
            }
        }
    }
}

#[test]
fn check() {
    let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    Solution::game_of_life(&mut board);
    assert_eq!(
        board,
        vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
    );
}
