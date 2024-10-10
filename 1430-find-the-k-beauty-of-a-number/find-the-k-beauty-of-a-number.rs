use std::str::from_utf8;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let str = num.to_string();
        let mut counter: i32 = 0;
        let k = k as usize;

        for i in 0..str.len() - k + 1 {
            counter += match &str[i..i+k].parse::<i32>() {
                Ok(n) => if *n != 0 && num % n == 0 { 1 } else { 0 }
                Err(e) => 0
            }
        }

        counter
    }
}