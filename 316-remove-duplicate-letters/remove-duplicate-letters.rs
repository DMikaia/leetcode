use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut seen: HashSet<char> = HashSet::new();
        let mut last_occ: HashMap<char, usize> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            last_occ.insert(c, i);
        }

        for (i, c) in s.chars().enumerate() {
            if !seen.contains(&c) {
                while let Some(&top) = stack.last() {
                    if c < top && i < *last_occ.get(&top).unwrap() {
                        seen.remove(&stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                seen.insert(c);
                stack.push(c);
            }
        }

        stack.into_iter().collect()
    }
}