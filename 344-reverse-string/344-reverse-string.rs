impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut start = 0;
        let mut end = s.len() - 1;

        while start < end {
            s.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}