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

                match token.as_str() {
                    "+" => {
                        stack.push(n_2 + n_1)
                    }
                    "-" => {
                        stack.push(n_2 - n_1);
                    }
                    "*" => {
                        stack.push(n_2 * n_1);
                    }
                    "/" => {
                        stack.push(n_2 / n_1);
                    }
                    _ => {}
                }
            } else {
                let num: i32 = token.parse().unwrap();
                stack.push(num);
            }

        }

        stack.pop().unwrap()
    }
}