struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::quick_sort(0, nums.len() - 1, &mut nums);
        nums
    }

    fn quick_sort(start: usize, end: usize, num: &mut Vec<i32>) {
        if start < end {
            let (mut i, mut j) = (start, end);
            let base = num[start];
            while i < j {
                while i < j && base > num[j] {
                    j -= 1;
                }
                num[i] = num[j];
                while i < j && base <= num[i] {
                    i += 1;
                }
                num[j] = num[i];
            }
            num[i] = base;
            Self::quick_sort(i + 1, end, num);
            if i > 1 {
                Self::quick_sort(start, i - 1, num);
            }
        }
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::sort_array(vec![1, 2, 5, 4, 0]),
        vec![5, 4, 2, 1, 0]
    );
    assert_eq!(Solution::sort_array(vec![0]), vec![0]);
    assert_eq!(Solution::sort_array(vec![0, 1]), vec![1, 0]);
}
