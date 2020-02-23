struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn exist(board:Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();
        if word.is_empty(){return true;}
        for (y,i) in board.iter().enumerate(){
            for (x,j) in i.iter().enumerate(){
                if j == &word[0]{
                    if Self::dfs(x as i32,y as i32,&mut board.clone(),0,&word){
                        return true;
                    }
                }
            }
        }
        false
    }

    fn dfs(x:i32,y:i32,board : &mut Vec<Vec<char>>,step:usize,word :&[char])->bool{
        if x<0 || x>=board[0].len() as i32 || y < 0 || y>=board.len() as i32||board[y as usize][x as usize] != word[step]{return false}
        let mut tmp = '/';
        if board[y as usize][x as usize] == word[step]{
            tmp = board[y as usize][x as usize];
            board[y as usize][x as usize] = '/';
            if step== word.len()-1{
                return true;
            }
        }
        let res = Self::dfs(x-1,y,board, step+1,word)||Self::dfs(x+1,y,board, step+1,word)||
                    Self::dfs(x,y-1,board, step+1,word)||Self::dfs(x,y+1,board, step+1,word);
        board[y as usize][x as usize] = tmp;
        res
    }
}


#[test]
fn check() {
    assert_eq!(Solution::exist(vec![vec!['a','b'],vec!['c','d']],"abdc".to_string()),true);
    assert_eq!(Solution::exist(vec![vec!['a','b'],vec!['c','d']],"abcd".to_string()),false);
    assert_eq!(Solution::exist(vec![vec!['a']],"a".to_string()),true);
}