pub struct Solution;

impl Solution {
    pub fn min_number(mut nums: Vec<i32>) -> String {
        Self::varient_quick_sort(0, nums.len() - 1, &mut nums);
        nums.iter().map(|i| i.to_string()).collect::<String>()
    }

    fn varient_quick_sort(start: usize, end: usize, nums: &mut Vec<i32>) {
        if start < end {
            let (mut i, mut j) = (start, end);
            let base = nums[start];
            while i != j {
                while i < j && Self::compare2(base, nums[j]) {
                    //从右往左 当找到比base小的交换
                    j -= 1;
                }
                nums[i] = nums[j];
                while i < j && Self::compare2(nums[i], base) {
                    //从左往右 当找到比base大的交换
                    i += 1;
                }
                nums[j] = nums[i];
            }
            nums[i] = base;
            if i > 0 {
                Self::varient_quick_sort(start, i - 1, nums);
            }
            Self::varient_quick_sort(i + 1, end, nums);
        }
    }

    fn compare2(origin: i32, other: i32) -> bool {
        origin.to_string() + &other.to_string() < other.to_string() + &origin.to_string()
    }

    fn compare(origin: i32, other: i32) -> bool {
        //该合并的 , 写半天写的什么东西。简直多此一举
        let s_origin = origin.to_string().chars().collect::<Vec<char>>();
        let s_other = other.to_string().chars().collect::<Vec<char>>();
        let mut i = 0;
        for v in s_origin.iter() {
            if let Some(ov) = s_other.get(i) {
                if *v == *ov {
                    i += 1;
                } else {
                    return *v < *ov;
                }
            } else if i > 0 {
                for k in 0..i {
                    if s_origin[k] == *v {
                        continue;
                    }
                    return *v <= s_other[k];
                }
            }
        }
        if s_other.len() > i {
            for k in 0..i {
                if s_other[k] == s_other[i] {
                    continue;
                }
                return s_other[i] > s_other[k];
            }
            return false;
        }
        true
    }
}

#[test]
fn check() {
    assert_eq!(Solution::compare(123, 321), true);
    assert_eq!(Solution::compare(123, 123), true);
    assert_eq!(Solution::compare(123, 12), false);
    assert_eq!(Solution::compare(3, 30), false);
    assert_eq!(Solution::compare(31, 30), false);
    assert_eq!(Solution::compare(30, 40), true);
    assert_eq!(Solution::compare(0, 1), true);
    assert_eq!(Solution::compare(0, 0), true);
    assert_eq!(Solution::compare(22, 2), true);
    assert_eq!(Solution::compare(2, 22), false);
    assert_eq!(Solution::compare(121, 12), true);
    assert_eq!(Solution::compare(12, 121), false);
    assert_eq!(Solution::compare(121, 121), true);
    assert_eq!(Solution::compare(403, 40), true);
    assert_eq!(Solution::compare(40, 403), false);

    assert_eq!(
        Solution::min_number(vec![3, 30, 34, 5, 9]),
        "3033459".to_string()
    );
    assert_eq!(Solution::min_number(vec![10, 2]), "102".to_string());
    assert_eq!(Solution::min_number(vec![121, 12]), "12112".to_string());
    assert_eq!(Solution::min_number(vec![12, 121]), "12112".to_string());
    assert_eq!(Solution::min_number(vec![403, 40]), "40340".to_string());
}
