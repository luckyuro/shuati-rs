use super::Solution;

impl Solution {
    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut pos = 0usize;
        // let mut f = 1usize;
        // let mut k = 1i32;
        // while f < nums.len() {
        //     if nums[pos] == nums[f] {
        //         f += 1;
        //     } else {
        //         pos += 1;
        //         nums[pos] = nums[f];
        //         f += 1;
        //         k += 1;
        //     }
        // }
        // k
        for f in 1..nums.len() {
            if nums[pos] != nums[f] {
                pos += 1;
                nums[pos] = nums[f]
            }
        }
        (pos + 1) as i32
    }
}
