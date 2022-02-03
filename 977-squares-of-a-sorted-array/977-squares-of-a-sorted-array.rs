impl Solution { 
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() - 1;
        let mut result = Vec::new();
        let (mut beg, mut end, mut i) = (0, n, n);

        loop {
            if nums[beg].pow(2) > nums[end].pow(2) {
                result.insert(0, nums[beg].pow(2));
                beg += 1
            } else {
                result.insert(0, nums[end].pow(2));
                if end != 0 {
                    end -= 1
                }
            }

            if i != 0 {
                i -= 1
            } else {
                break
            }
        }

        result
    }
}