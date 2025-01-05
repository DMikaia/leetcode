impl Solution {
    pub fn simplify_path(path: String) -> String {
        let elements = path.split('/');
        let mut stack: Vec<&str> = vec![];

        for e in elements {
            match e {
                ".." => {
                    if !stack.is_empty() { 
                        stack.pop(); 
                    } 
                },
                "" => {},
                "." => {},
                _ => {
                    stack.push(e);
                }
            }
        }

        let mut result = String::new();
        result.push('/');
        for (i, e) in stack.iter().enumerate() {
            if (i > 0) {
                result.push('/');
            }
            result.push_str(e)
        }

        result
    }
}