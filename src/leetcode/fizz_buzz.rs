struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|n| {
                if n < 5 {
                    if n == 3 {
                        return String::from("Fizz");
                    } else {
                        return n.to_string();
                    }
                }
                match (n % 3, n % 5) {
                    (0, 0) => String::from("FizzBuzz"),
                    (0, _) => String::from("Fizz"),
                    (_, 0) => String::from("Buzz"),
                    _ => n.to_string(),
                }
            })
            .collect::<Vec<String>>()
    }
}
