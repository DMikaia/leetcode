impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();

        for part in path.trim_start_matches('/').split('/') {
            match part {
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                }
                "." | "" => {}
                _ => {
                    stack.push(part);
                }
            }
        }

        if stack.is_empty() {
            return "/".to_string(); 
        }

        let mut result = String::new();
        for (i, part) in stack.iter().enumerate() {
            if i > 0 {
                result.push('/');
            }
            result.push_str(part);
        }

        format!("/{}", result)
    }
}