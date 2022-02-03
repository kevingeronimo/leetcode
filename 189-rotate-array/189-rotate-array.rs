impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            if let Some(v) = nums.pop() {
                nums.insert(0, v)
            }
        }
    }
}