impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.binary_search(&target).is_ok() {
            vec![
                nums.partition_point(|item| *item < target) as i32,
                nums.partition_point(|item| *item <= target) as i32 - 1
            ]
        } else {
            vec![-1, -1]
        }
    }
}