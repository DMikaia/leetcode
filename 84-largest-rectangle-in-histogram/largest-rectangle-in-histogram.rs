impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut max_area = 0;

        for i in 0..heights.len() {
            let mut start = i;

            while !stack.is_empty() && heights[i] < stack[stack.len() -1].1  
            {
                let last = stack.pop().unwrap();
                max_area = max_area.max(last.1 * (i - last.0) as i32);
                start = last.0;
            }

            stack.push((start, heights[i]));
        }

        while let Some(top) = stack.pop()  {
            max_area = max_area.max(top.1 * (heights.len() - top.0) as i32);
        }

        return max_area;
    }
}