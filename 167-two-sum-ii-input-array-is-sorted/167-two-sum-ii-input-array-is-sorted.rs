impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pivot = 0;
        let mut pointer = pivot + 1;

        loop {
            if numbers[pivot] + numbers[pointer] == target {
                break vec![pivot as i32 + 1, pointer as i32 + 1]
            } else if pointer == numbers.len() - 1 {
                pivot += 1;
                pointer = pivot + 1;
            }
            else {
                pointer += 1
            };
        }
    }
}