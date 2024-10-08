impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        let mut avg = 0.0;
        let k = k as usize;
        let kf = k as f64;

        for i in 0..k {
            sum += nums[i];
        }

        avg = sum as f64 / kf;

        for i in k..nums.len() {
            sum += nums[i];
            sum -= nums[i - k];

            avg = avg.max(sum as f64 / kf);
        }

        avg
    }
}