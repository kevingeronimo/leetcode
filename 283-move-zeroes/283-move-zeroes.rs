impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut p_zero = 0;
        let mut p_nzero = 0;
        let n = nums.len();

        while p_nzero < n {
            while nums[p_zero] != 0 && (p_zero < n - 1) {
                p_zero += 1;
            }

            if (p_nzero > p_zero) && (nums[p_nzero] != 0) {
                nums.swap(p_zero, p_nzero) 
            }

            p_nzero += 1;

        }
    }
}