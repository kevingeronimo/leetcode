impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        fn f(mut i: usize, j: usize, s: &[char]) -> i32 {
            let (mut ans, mut bal) = (0, 0);

            let mut range = i..j;

            while let Some(k) = range.next() {
                if s[k] == '(' {
                    bal += 1
                } else {
                    bal -= 1
                }

                if bal == 0 {
                    if k - i == 1 {
                        ans += 1
                    } else {
                        ans += 2 * f(i + 1, k, s)
                    }

                    i = k + 1;
                }
            }

            ans
        }

        f(0, s.len(), &s)
    }
}