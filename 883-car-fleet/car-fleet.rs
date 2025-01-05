impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pair: Vec<(&i32, &i32)> = position.iter()
            .zip(speed.iter())
            .collect();
        
        pair.sort_by(|a, b| b.0.cmp(&a.0));

        let mut stack: Vec<f32> = Vec::new();
        for p in pair {
            stack.push((target - p.0) as f32 / *p.1 as f32);
            if stack.len() >= 2 && stack.last() <= stack.get(stack.len() - 2) {
                stack.pop();
            }
        }

        stack.len() as i32
    }
}