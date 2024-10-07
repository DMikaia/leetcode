impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut valid = false;
        let rows = matrix.len() as i32;
        
        if rows == 0 || matrix[0].len() == 0 {
            return false;
        }

        let cols = matrix[0].len() as i32;
        let mut start = 0;
        let mut end = (rows * cols) - 1;

        while start <= end {
            let mid = start + (end - start) / 2;
            let mid_value = matrix[(mid / cols) as usize][(mid % cols) as usize];

            if mid_value < target {
                start = mid + 1;
            } else if mid_value == target {
                valid = true;
                break;
            } else {
                end = mid - 1;
            }
        }
        
        valid
    }
}
