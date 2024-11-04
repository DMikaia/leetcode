impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left as i32 + (right - left) as i32 / 2;

            if nums[mid as usize] == target {
                return mid;
            }

            if nums[left] <= nums[mid as usize] {
                if nums[left] <= target && target <= nums[mid as usize] {
                    right = mid as usize - 1;
                } else {
                    left = mid as usize + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[right] {
                    left = mid as usize + 1;
                } else {
                    right = mid as usize - 1;
                }
            }
        }

        -1
    }
}