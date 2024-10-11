impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len();
        let mut result: Vec<i32> = vec![0; len];

        if k == 0 {
            result
        } else if k > 0 {
            let k = k as usize;

            let mut sum = code[1..=k].iter().sum::<i32>();

            for i in 1..len + 1 {
                result[i - 1] = sum;
                
                sum -= code[i % len];
                sum += code[(len + i + k) % len];
            }

            result
        } else {
            let k = -k as usize; 
            let mut sum = code[len - k..].iter().sum();

            for i in 1..len + 1 {
                result[i - 1] = sum;

                sum -= code[(len - 1 + i - k) % len];
                sum += code[(i - 1)  % len];
            }
            
            result
        }
    }
}
