use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        nums.iter().for_each(|&num| *map.entry(num).or_insert(0) += 1);
        map.iter().fold(0, |acc, (&num, &count)| {
            map.get(&(num + 1)).map_or(acc, |c| acc.max(count + c))
        })
    }
}