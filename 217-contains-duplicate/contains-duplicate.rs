use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();

        for num in nums {
            if map.contains(&num) {
                return true;
            }
            map.insert(num);
        }

        false
    }
}