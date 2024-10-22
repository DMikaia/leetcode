use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let char_vec: Vec<char> = s.chars().collect();
        let mut map = HashMap::new();
        let mut max_length: i32 = 0;
        let mut start = 0;

        for end in 0..char_vec.len() {
            let right_char = char_vec[end];

            if let Some(value) = map.get(&right_char) {
                start = cmp::max(start, value + 1); 
            }

            map.insert(right_char, end);
            max_length = cmp::max(max_length, (end - start + 1) as i32);
        }

        max_length
    }
}