impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut low = 0;
        let mut high = numbers.len() - 1;
        let mut output = Vec::new();

        while low < high {
            if numbers[low] + numbers[high] == target {
                output.push(low as i32 + 1);
                output.push(high as i32 + 1);
                break
            } else if numbers[low] + numbers[high] < target {
                low += 1
            } else {
                high -= 1
            };
        }

        output
    }
}