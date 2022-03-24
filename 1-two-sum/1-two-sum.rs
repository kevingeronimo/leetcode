use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut visited = HashMap::new();
        let mut i1 = 0;
        let mut i2 = 0;

        for (i, n) in nums.iter().enumerate() {
            let req_value = target - n;
            if visited.contains_key(&req_value) {
                i1 = *visited.get(&req_value).unwrap() as i32;
                i2 = i as i32;
            } else {
                visited.insert(n, i);
            }
        }

        vec![i1, i2]
    }
}