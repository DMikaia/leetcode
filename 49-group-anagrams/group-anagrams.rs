use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for str in strs {
            let mut key: Vec<char> = str.chars().collect();
            key.sort();

            map.entry(key).or_insert(Vec::new()).push(str);
        }

        map.values().cloned().collect()
    }
}