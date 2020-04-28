struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut used = vec![false; nums.len()];
        let mut path = vec![];
        Self::dfs(&mut path, &nums, &mut used, &mut res);
        res
    }

    fn dfs(path: &mut Vec<i32>, nums: &Vec<i32>, used: &mut Vec<bool>, res: &mut Vec<Vec<i32>>) {
        let mut finished = true;
        for i in 0..used.len() {
            //找到未被使用过的
            if !used[i] {
                //不是最低点
                finished = false;
                used[i] = true;
                path.push(nums[i]);
                Self::dfs(path, nums, used, res);
                path.remove(path.len() - 1);
                used[i] = false;
            }
        }
        if finished {
            res.push(path.clone());
        }
    }
}
