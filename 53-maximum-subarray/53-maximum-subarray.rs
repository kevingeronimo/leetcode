impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = i32::MIN;

        for i in nums {
            sum += i;

            if max < sum {
                max = sum
            }

            if sum < 0 {
                sum = 0
            }
        }

        max
    }
}