impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut water = 0;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            if height[left] < height[right] {
                if left_max > height[left] {
                    water += left_max - height[left];
                } else {
                    left_max = height[left];
                }

                left += 1;
            } else {
                if right_max > height[right] {
                    water += right_max - height[right];
                } else {
                    right_max = height[right];
                }

                right -= 1;
            }
        }

        water
    }
}