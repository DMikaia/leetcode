impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();

        if n == 1 {
            return vec![0];
        }

        let mut result: Vec<i32> = vec![0; n]; 

        for i in (0..=n - 2).rev() {
            let mut pointer = i + 1;
            let mut counter = 1;

            while temperatures[i] >= temperatures[pointer] {
                if pointer == n - 1 || result[pointer] == 0 {
                    counter = 0;
                    break;
                }

                counter += result[pointer];
                pointer += result[pointer] as usize;
            }

            result[i] = counter;
        }

        result
    }
}