use super::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut current_acc = 0;
        for n in nums.iter() {
            current_acc += n;
            max = if current_acc < max {max} else {current_acc};
            if current_acc < 0 {
                current_acc = 0;
            }
        }
        max
    }
}
