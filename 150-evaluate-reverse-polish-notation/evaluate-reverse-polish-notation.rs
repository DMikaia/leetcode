impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let n_1: i32 = stack.pop().unwrap();
                    let n_2: i32 = stack.pop().unwrap();

                    stack.push(n_2 + n_1);
                }
                "-" => {
                    let n_1: i32 = stack.pop().unwrap();
                    let n_2: i32 = stack.pop().unwrap();

                    stack.push(n_2 - n_1);
                }
                "*" => {
                    let n_1: i32 = stack.pop().unwrap();
                    let n_2: i32 = stack.pop().unwrap();

                    stack.push(n_2 * n_1);
                }
                "/" => {
                    let n_1: i32 = stack.pop().unwrap();
                    let n_2: i32 = stack.pop().unwrap();

                    stack.push(n_2 / n_1);
                }
                _ => {
                    let t_int: i32 = token.trim().parse().unwrap();
                    stack.push(t_int);
                }
            }
        }

        stack.pop().unwrap()
    }
}