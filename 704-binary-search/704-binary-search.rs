impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut min: usize = 0;
        let mut max: usize = nums.len();

        loop {
            let mid = min + (max - min) / 2;

            // check first guess
            if target == nums[mid] {
                break mid as i32;
                
            // min = mid when integer not found
            } else if min == mid {
                break -1;
                
            // removes right half
            } else if target < nums[mid] {
                max = mid
                
            // removes left half
            } else {
                min = mid
            };
        }
    }
}