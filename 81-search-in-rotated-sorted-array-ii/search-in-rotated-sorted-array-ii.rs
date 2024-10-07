impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len();
        let (mut l, mut r) = (0, n - 1);
        while l <= r {
            while l < r && nums[l] == nums[r] {
                l += 1;
            }
            let m = l + (r - l) / 2;
            if nums[m] == target {
                return true;
            }
            if nums[l] <= nums[m] {
                if nums[l] <= target && target < nums[m] {
                    if m >= 1 {
                        r = m - 1;
                    } else {
                        return false;
                    }
                } else {
                    l = m + 1;
                }
            } else {
                if nums[m] < target && target <= nums[r] {
                    l = m + 1;
                } else {
                    if m >= 1 {
                        r = m - 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        false
    }
}