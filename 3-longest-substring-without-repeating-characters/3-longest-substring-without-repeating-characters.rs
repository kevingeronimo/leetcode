use std::collections::HashSet;
use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();

        let mut i = 0;
        let mut j = 0;
        let mut max = 0;

        let mut hash_set = HashSet::new();

        while j < s.len() {
            if !hash_set.contains(&chars[j]) {
                hash_set.insert(&chars[j]);
                j += 1;
                max = cmp::max(hash_set.len(), max);
            } else {
                hash_set.remove(&chars[i]);
                i += 1;
            }
        }

        max as i32
    }
}