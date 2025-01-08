use std::collections::{HashMap};

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut result: String = String::new();
        let mut last_occ: HashMap<char, usize> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            last_occ.insert(c, i);
        }

        for (i, c) in s.chars().enumerate() {
            if !result.contains(c) {
                while let Some(top) = result.chars().nth(result.len() - 1) {
                    if c < top && i < *last_occ.get(&top).unwrap() {
                        result.remove(result.len() - 1);
                    } else {
                        break;
                    }
                }

                result.push(c);
            }
        }

        result
    }
}