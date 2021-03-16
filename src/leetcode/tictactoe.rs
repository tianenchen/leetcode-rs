struct Solution;
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        if moves.len() < 5 {
            return "Pending".to_owned();
        }
        let mut graph = vec![vec![' '; 3]; 3];
        for (i, v) in moves.iter().enumerate() {
            graph[v[0] as usize][v[1] as usize] = if i % 2 == 0 { 'X' } else { 'O' };
        }

        for i in 0..3{
            let mut positive = 0;
            let mut nagative = 0;
            for j in 0..3{
                if graph[i][j] == 'X'{
                    positive+=1;
                }
                else if graph[i][j] == 'O'{
                    nagative+=1;
                }
            }
            if positive == 3{
                return "A".to_owned();
            }
            if nagative == 3{
                return "B".to_owned();
            }
            positive = 0;
            nagative = 0;
            for j in 0..3{
                if graph[j][i] == 'X'{
                    positive+=1;
                }
                else if graph[j][i] == 'O'{
                    nagative+=1;
                }
            }
            if positive == 3{
                return "A".to_owned();
            }
            if nagative == 3{
                return "B".to_owned();
            }
        }

        let mut positive1 = 0;
        let mut nagative1 = 0;

        let mut positive2 = 0;
        let mut nagative2 = 0;
        for i in 0..3{
            for j in 0..3{
                if i == j{
                    if graph[i][j] == 'X'{
                        positive1+=1;
                    }
                    else if graph[i][j] == 'O'{
                        nagative1+=1;
                    }
                }

                if i == 2-j{
                    if graph[i][j] == 'X'{
                        positive2+=1;
                    }
                    else if graph[i][j] == 'O'{
                        nagative2+=1;
                    }
                }
            }
        }

        if positive1 == 3 || positive2 == 3 {
            return "A".to_owned();
        }
        if nagative1 == 3 || nagative2 == 3{
            return "B".to_owned();
        }
        if moves.len() < 9 {
            return "Pending".to_owned();
        }
        "Draw".to_owned()
    }
}

#[test]
fn check() {
    Solution::tictactoe(vec![vec![0,0],vec![2,0],vec![1,1],vec![2,1],vec![2,2]]);
}