struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        let len = nums.len();
        let mut sorted = vec![];
        for i in 0..len {
            let v = nums[i];
            let index;
            if sorted.len() < k + 1 {
                index = Self::binary_search(v, &sorted);
                sorted.insert(index, v);
            } else {
                let weed = nums[i - k - 1];
                let weed_index = Solution::binary_search(weed, &sorted);
                if weed_index > 0 {
                    sorted.remove(weed_index - 1);
                }
                index = Self::binary_search(v, &sorted);
                sorted.insert(index, v);
            }
            if let Some(back) = sorted.get(index + 1) {
                if *back as i64 - v as i64 <= t as i64 {
                    return true;
                }
            }
            if index > 0 && sorted.get(index - 1).is_some() {
                if v as i64 - sorted[index - 1] as i64 <= t as i64 {
                    return true;
                }
            }
        }
        false
    }

    fn binary_search(v: i32, sorted: &Vec<i32>) -> usize {
        let (mut start, mut end) = (0, sorted.len());
        while start < end {
            let mid = end + start >> 1;
            if v >= sorted[mid] {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        start
    }

    fn quick_sort(start: usize, end: usize, nums: &mut Vec<i32>) {
        if start < end {
            let (mut i, mut j) = (start, end);
            let base = nums[i];
            while i < j {
                while i < j && nums[j] > base {
                    j -= 1;
                }
                nums[i] = nums[j];
                while i < j && nums[i] <= base {
                    i += 1;
                }
                nums[j] = nums[i];
            }
            nums[i] = base;
            if i > 1 {
                Self::quick_sort(start, i - 1, nums);
            }
            Self::quick_sort(i + 1, end, nums);
        }
    }
}

#[test]
fn check() {
    let mut v = vec![1, 2, 3, 4, 6, 5];
    Solution::quick_sort(0, v.len() - 1, &mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    let mut v = vec![1];
    Solution::quick_sort(0, v.len() - 1, &mut v);
    assert_eq!(v, vec![1]);
    let mut v = vec![2, 1];
    Solution::quick_sort(0, v.len() - 1, &mut v);
    assert_eq!(v, vec![1, 2]);

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(Solution::binary_search(4, &v), 4);

    let v = vec![1];
    assert_eq!(Solution::binary_search(1, &v), 1);

    let v = vec![];
    assert_eq!(Solution::binary_search(1, &v), 0);

    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
        true
    );
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
        false
    );
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![-1, 2147483647], 1, 2147483647),
        false
    );
}
