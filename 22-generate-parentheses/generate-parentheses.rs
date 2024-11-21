impl Solution {
    fn backtrack(open_n: i32, close_n: i32, n: i32, res: &mut Vec<String>, stack: &mut Vec<String>) {
        if open_n == close_n && open_n == n {
            res.push(stack.join(""));
            return;
        }

        if open_n < n {
            stack.push("(".to_string());
            Self::backtrack(open_n + 1, close_n, n, res, stack);
            stack.pop();
        }

        if close_n < open_n {
            stack.push(")".to_string());
            Self::backtrack(open_n, close_n + 1, n, res, stack);
            stack.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut stack = Vec::new();

        Self::backtrack(0, 0, n, &mut res, &mut stack);

        res
    }
}