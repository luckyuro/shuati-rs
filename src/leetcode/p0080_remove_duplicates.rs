use super::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut pos = 1;
        let mut repeat = 1;
        for i in 1..nums.len() {
            if nums[pos - 1] == nums[i] {
                if repeat < 2 {
                    nums[pos] = nums[i];
                    pos += 1;
                }
                repeat += 1;
            } else {
                nums[pos] = nums[i];
                pos += 1;
                repeat = 1;
            }
        }
        pos as i32
    }
}
