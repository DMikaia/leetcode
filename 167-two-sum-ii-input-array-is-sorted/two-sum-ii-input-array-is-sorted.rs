use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);

        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => return vec![(left+1) as i32, (right+1) as i32],
            }
        }
        vec![]
    }
}