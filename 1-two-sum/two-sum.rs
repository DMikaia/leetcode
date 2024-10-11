use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let num = nums[i];
            let diff = target - num;

            if let Some(value) = map.get(&diff) {
                return vec![*value, i as i32];
            }

            map.insert(num, i as i32);
        }

        return vec![0, 0];
    }
}