struct Solution;

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut set = std::collections::HashSet::new();
        for i in nums {
            if set.contains(&i) {
                set.remove(&i);
            } else {
                set.insert(i);
            }
        }
        set.into_iter().collect::<Vec<i32>>()
    }

    fn get_single_numbers_by_xor(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        for i in &nums {
            xor ^= *i;
        }
        // let m = xor & -xor;
        let m = {
            let mut count = 1;
            while xor & count != count {
                count <<= 1;
            }
            count
        };
        let mut res = vec![0; 2];
        for i in nums {
            if m & i == 0 {
                res[0] ^= i;
            } else {
                res[1] ^= i;
            }
        }
        res
    }
}

#[test]
fn check() {
    // let mut xor = 8;
    // let m = {
    //     let mut count = 1;
    //     while xor & count == 1{
    //         count<<=1;
    //     }
    //     count
    // };
    // println!("{:b}", 8^(-8) );
    // println!("{:b}", 8);
    // println!("{:b}", -8);
    // println!("{:b}", m );
}
