use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        let mut answer = false;

        for i in nums {
            if !set.contains(&i) {
                set.insert(i);
            } else {
                answer = true;
            }
        }

        answer
    }
}