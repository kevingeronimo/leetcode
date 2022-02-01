impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut min = 0;
        let mut max = nums.len();

        loop {
            let mid = min + ((max - min) / 2);

            if nums[mid] == target {
                break mid as i32;
            } else if ((max - min) == 1) && (target < nums[min]) {
                // check if target is less when two values
                break min as i32;
            } else if (max - min) == 1 {
                // when two values remain, target is missing
                break max as i32
            }
            else if nums[mid] < target {
                min = mid
            } else {
                max = mid
            };
        }
    }
}