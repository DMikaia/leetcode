impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len();
        let mut result: Vec<i32> = vec![0; len];

        if k == 0 {
            result
        } else if k > 0 {
            let k = k as usize;
            let mut sum = 0;

            for i in 0..k {
                sum += code[i];
            }

            for i in 0..len {
                sum -= code[i];
                sum += code[(i + k) % len];

                result[i] = sum;
            }

            result
        } else {
            let k = -k as usize;
            let mut sum = 0;

            for i in 1..k+1 {
                sum += code[len - 1 - i];
            }

            for i in 0..len {
                sum -= code[(len - 1 + i - k) % len];
                sum += code[(len - 1 + i)  % len];

                result[i] = sum;
            }

            result
        }
    }
}