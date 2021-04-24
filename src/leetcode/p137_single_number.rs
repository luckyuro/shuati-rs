use super::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bit_count :Vec<u8> =vec![0; 32];
        for x in nums {
            let mut _in_x = x;
            let mut pos = 32;
            while pos > 0 {
                let b = _in_x & 1;
                bit_count[pos-1] = (bit_count[pos-1] + b as u8) % 3;

                _in_x >>= 1;
                pos -= 1;
            }
        }
        let mut ans = 0;
        for x in bit_count {
            ans <<= 1;
            ans += x as i32;
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::single_number(vec![2,2,3,2]), 3);
        assert_eq!(Solution::single_number(vec![0,1,0,1,0,1,99]), 99);
        assert_eq!(Solution::single_number(vec![-2,-2,1,1,-3,1,-3,-3,-4,-2]), -4);
    }
}