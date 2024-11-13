use std::cmp::max;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut water = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = height[left];
        let mut right_max = height[right];

        while left < right {
            if left_max < right_max {
                left += 1;
                left_max = max(left_max, height[left]);
                water += left_max - height[left];
            } else {
                right -= 1;
                right_max = max(right_max, height[right]);
                water += right_max - height[right];
            }
        }

        water
    }
}