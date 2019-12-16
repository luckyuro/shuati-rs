struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for x in nums.iter() {
            ans ^= *x;
        }
        ans
    }
}