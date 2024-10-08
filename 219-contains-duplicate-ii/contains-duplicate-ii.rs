use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut window = HashSet::new();
        let mut left: i32 = 0;

        for (right, e) in nums.iter().enumerate() {
            if (right as i32) - left > k {
                window.remove(&nums[left as usize]);
                left += 1;
            }
            if window.contains(e) {
                return true;
            }
            window.insert(e.clone());
        }

        false
    }
}