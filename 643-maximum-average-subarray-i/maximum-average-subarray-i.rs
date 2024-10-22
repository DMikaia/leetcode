use std::cmp;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut avg: f64 = std::f64::MIN;
        let mut sum = 0;
        let mut start = 0;

        for end in 0..nums.len() {
            sum += nums[end];

            if end as i32 >= k - 1 as i32 {
                avg = avg.max(sum as f64 / k as f64);
                sum -= nums[start];
                start += 1;
            }
        }

        avg
    }
}