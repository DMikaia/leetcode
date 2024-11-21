impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, i32)> = Vec::new();

        for (i, &current_temp) in temperatures.iter().enumerate() {
            while !stack.is_empty() && current_temp > stack[stack.len() - 1].0 {
                let pair = stack.pop().unwrap();

                result[pair.1 as usize] = i as i32 - pair.1;
            }

            stack.push((current_temp, i as i32));
        }

        result
    }
}