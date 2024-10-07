impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {        
        let (mut start, mut end): (i32, i32) = (0, (nums.len() as i32) - 1);

        while start <= end {
            let mid = start + (end - start) / 2;

            if nums[mid as usize] < target {
                start = mid + 1;
            } else if nums[mid as usize] == target {
                return mid;
            } else {
                end = mid - 1;
            }
        }

        return start;
    }
}