impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        fn is_operator(s: &str) -> bool {
            s == "+" || s == "-" || s == "*" || s == "/"
        }

        for token in tokens {
            if is_operator(&token) {
                let n_1: i32 = stack.pop().unwrap();
                let n_2: i32 = stack.pop().unwrap();

                let result = match token.as_str() {
                    "+" => n_2 + n_1,
                    "-" => n_2 - n_1,
                    "*" => n_2 * n_1,
                    _ => n_2 / n_1,
                };

                stack.push(result);
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }

        }

        stack.pop().unwrap()
    }
}