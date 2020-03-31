/*

给定整数数组 A，每次 move 操作将会选择任意 A[i]，并将其递增 1。

返回使 A 中的每个值都是唯一的最少操作次数。

示例 1:

输入：[1,2,2]
输出：1
解释：经过一次 move 操作，数组将变为 [1, 2, 3]。
示例 2:

输入：[3,2,1,2,1,7]
输出：6
解释：经过 6 次 move 操作，数组将变为 [3, 4, 1, 2, 5, 7]。
可以看出 5 次或 5 次以下的 move 操作是不能让数组的每个值唯一的。

*/
struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut a: Vec<i32>) -> i32 {
        if a.is_empty() {
            return 0;
        }
        let len = a.len();
        Self::quick_sort(0, len - 1, &mut a);
        // dbg!(a.clone());
        let mut res = 0;
        for i in 1..len {
            let delta = a[i] - a[i - 1];
            if delta <= 0 {
                a[i] += -delta + 1;
                res += -delta + 1;
            }
        }
        // dbg!(a.clone());
        res
    }
    fn quick_sort(start: usize, end: usize, a: &mut Vec<i32>) {
        if start < end {
            let base = a[start];
            let (mut i, mut j) = (start, end);
            while i < j {
                while i < j && base <= a[j] {
                    //从后往前找  找到比base 大的就交换
                    j -= 1;
                }
                a[i] = a[j];
                while i < j && base > a[i] {
                    i += 1;
                }
                a[j] = a[i];
            }
            a[i] = base;
            if i > 1 {
                Self::quick_sort(start, i - 1, a);
            }
            Self::quick_sort(i + 1, end, a);
        }
    }
    //写复杂了， 应该先排序，然后遍历保持当前数比前一个数大一就行了
    pub fn min_increment_for_unique3(mut a: Vec<i32>) -> i32 {
        if a.is_empty() {
            return 0;
        }
        let len = a.len();
        let mut dp = vec![0; len];
        let mut visited = vec![a[0]];
        for i in 1..len {
            // dbg!(visited.clone());
            let mut tmp = 0;
            let idx = Self::binary_search(a[i], &visited);
            let len = visited.len();
            while idx + tmp < len && visited[idx + tmp] == a[i] + tmp as i32 {
                tmp += 1;
            }
            visited.insert(idx + tmp, a[i] + tmp as i32);
            dp[i] = dp[i - 1] + tmp as i32;
        }
        dp[len - 1]
    }
    fn binary_search(v: i32, visited: &Vec<i32>) -> usize {
        let (mut start, mut end) = (0, visited.len());
        while start < end {
            let mid = start + end >> 1;
            if v > visited[mid] {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        start
    }
    pub fn min_increment_for_unique2(mut a: Vec<i32>) -> i32 {
        if a.is_empty() {
            return 0;
        }
        let len = a.len();
        let mut dp = vec![0; len];
        let mut visited = std::collections::HashSet::new();
        visited.insert(a[0]);
        for i in 1..len {
            let mut tmp = 0;
            while visited.contains(&(a[i] + tmp)) {
                tmp += 1;
            }
            visited.insert(a[i] + tmp);
            dp[i] = dp[i - 1] + tmp;
        }
        dp[len - 1]
    }
}

#[test]
fn check() {
    let mut v1 = vec![3, 2, 1, 0, 4, 9, -1];
    Solution::quick_sort(0, v1.len() - 1, &mut v1);
    assert_eq!(v1, vec![-1, 0, 1, 2, 3, 4, 9]);
    let mut tmp = vec![1, 2, 3, 5, 6];
    tmp.insert(3, 4);
    assert_eq!(tmp, vec![1, 2, 3, 4, 5, 6]);
    tmp.insert(6, 7);
    assert_eq!(tmp, vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(1 + 1 >> 1, 1);
    assert_eq!(Solution::binary_search(3, &vec![1, 2, 3, 4]), 2);
    assert_eq!(Solution::binary_search(7, &vec![1, 2, 3, 4, 5, 6, 7]), 6);
    assert_eq!(Solution::binary_search(1, &vec![1]), 0);
    assert_eq!(Solution::binary_search(4, &vec![1, 2, 5, 6, 7]), 2);
    assert_eq!(Solution::binary_search(5, &vec![1, 2, 5, 6, 7]), 2);
    assert_eq!(Solution::binary_search(4, &vec![1, 2, 2, 6, 7]), 3);
    assert_eq!(Solution::binary_search(8, &vec![1, 2, 2, 6, 7]), 5);
    assert_eq!(Solution::binary_search(-1, &vec![1, 2, 2, 6, 7]), 0);
    assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
    assert_eq!(Solution::min_increment_for_unique(vec![1, 1]), 1);
    assert_eq!(
        Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]),
        6
    );
}
