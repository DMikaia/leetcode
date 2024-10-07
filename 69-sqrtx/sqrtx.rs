impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }

        let mut start: i32 = 1;
        let mut end: i32 = x;
        let mut mid: i32 = -1;

        while start <= end {
            mid = start + (end - start) / 2;

            if mid <= x / mid {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }

        end
    }
}