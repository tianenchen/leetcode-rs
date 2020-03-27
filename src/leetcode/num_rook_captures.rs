struct Solution;
impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        for i in 0..8{
            for j in 0..8{
                if board[i][j] == 'R'{
                    let mut x = i;
                    let mut y = j;
                    while x+1<8 && board[x+1][y]!='B'{
                        if board[x+1][y]=='p'{
                            res+=1;
                            break;
                        }
                        x+=1;
                    }
                    x = i;
                    y = j;
                    while y+1<8 && board[x][y+1]!='B'{
                        if board[x][y+1]=='p'{
                            res+=1;
                            break;
                        }
                        y+=1;
                    }
                    x = i;
                    y = j;
                    while x>0 && board[x-1][y]!='B'{
                        if board[x-1][y]=='p'{
                            res+=1;
                            break;
                        }
                        x-=1;
                    }
                    x = i;
                    y = j;
                    while y>0 && board[x][y-1]!='B'{
                        if board[x][y-1]=='p'{
                            res+=1;
                            break;
                        }
                        y-=1;
                    }
                    break;
                }
            }
        }       
        res
    }
}