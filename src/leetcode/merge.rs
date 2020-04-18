struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = intervals.len();
        if len == 0 {
            return vec![];
        } else if len == 1 {
            return intervals;
        }
        Self::quick_sort(0, intervals.len() - 1, &mut intervals);
        let mut res = vec![];
        let mut i = 0;
        while i < len {
            if i < len - 1 && intervals[i][1] >= intervals[i + 1][1] {
                intervals[i + 1] = intervals[i].clone();
            } else if i < len - 1
                && intervals[i][1] >= intervals[i + 1][0]
                && intervals[i][1] <= intervals[i + 1][1]
            {
                intervals[i + 1] = vec![intervals[i][0], intervals[i + 1][1]];
            } else {
                res.push(intervals[i].clone());
            }
            i += 1;
        }
        res
    }

    fn quick_sort(start: usize, end: usize, intervals: &mut Vec<Vec<i32>>) {
        if start < end {
            let (mut i, mut j) = (start, end);
            let base = intervals[i].clone();
            while i < j {
                while i < j && !Self::cmp(&base, &intervals[j]) {
                    j -= 1;
                }
                intervals[i] = intervals[j].clone();
                while i < j && Self::cmp(&base, &intervals[i]) {
                    i += 1;
                }
                intervals[j] = intervals[i].clone();
            }
            intervals[i] = base;
            if i > 1 {
                Self::quick_sort(start, i - 1, intervals);
            }
            Self::quick_sort(i + 1, end, intervals);
        }
    }

    //letf >= right true
    fn cmp(l: &Vec<i32>, r: &Vec<i32>) -> bool {
        if l[0] > r[0] {
            true
        } else if l[0] < r[0] {
            false
        } else {
            if l[1] >= r[1] {
                true
            } else {
                false
            }
        }
    }
}

#[test]
fn check() {
    let mut v = vec![vec![0, 1], vec![2, 3], vec![1, 2]];
    Solution::quick_sort(0, v.len() - 1, &mut v);
    assert_eq!(v, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let mut v = vec![vec![0, 1]];
    Solution::quick_sort(0, v.len() - 1, &mut v);
    assert_eq!(v, vec![vec![0, 1]]);

    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![1, 5]]),
        vec![vec![1, 5]]
    );
    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![2, 3]]),
        vec![vec![1, 4]]
    );
}
