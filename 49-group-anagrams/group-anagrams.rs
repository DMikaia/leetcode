use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort();
            let sorted_str = chars.into_iter().collect();

            map.entry(sorted_str).or_insert(Vec::new()).push(str);
        }

        return map.into_values().collect();
    }
}