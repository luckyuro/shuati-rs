use super::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // not pass:
        // fn search_recursive(
        //     nums: &Vec<i32>,
        //     left: usize,
        //     right: usize,
        //     target: i32,
        // ) -> Option<usize> {
        //     let mid = (left + right) / 2;
        //     if left == right && nums[left] != target {
        //         None
        //     } else if nums[mid] == target {
        //         Some(mid)
        //     } else if nums[mid] < target && nums[left] < target {
        //         search_recursive(nums, left, mid + 1, target)
        //     } else {
        //         search_recursive(nums, mid + 1, right, target)
        //     }
        // }
        // search_recursive(&nums, 0, nums.len() - 1, target)
        //     .map(|x| x as i32)
        //     .unwrap_or(-1)

        let mut left = 0usize;
        let mut right = nums.len();
        while left != right {
            let mid = (left + right) / 2;
            println!("{},{},{}", left, right, mid);
            if nums[mid] == target {
                return mid as i32;
            } else if nums[left] < nums[mid] {
                if nums[left] <= target && nums[mid] > target {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[right - 1] >= target && nums[mid] < target {
                    left = mid + 1
                } else {
                    right = mid
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_search() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![1, 2], 0), -1);
    }
}
