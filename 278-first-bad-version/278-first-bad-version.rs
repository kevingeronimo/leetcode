// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, mut n: i32) -> i32 {
        let mut min = 0;
        let mut max = n;

        loop {
            let mid = min + ((max - min) / 2);

            if self.isBadVersion(mid) {
                max = mid
            } else {
                min = mid
            };

            // when max - min = 1, min is good and max is bad so we return max
            if (max - min) == 1 {
                break max
            }
        }
    }
}