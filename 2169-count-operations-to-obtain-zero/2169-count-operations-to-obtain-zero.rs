impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut operations = 0;

        while num1 > 0 && num2 > 0 {
            if num1 >= num2 {
                num1 -= num2;
                operations += 1
            } else {
                std::mem::swap(&mut num1, &mut num2)
            }
        }

        operations
    }
}