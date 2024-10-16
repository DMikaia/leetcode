use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        for &num in &set {
            if !set.contains(&(num - 1)) {
                let mut length = 1;

                while set.contains(&(num + length)) {
                    length += 1;
                }

                longest = max(longest, length);
            }
        }

        longest
    }
}