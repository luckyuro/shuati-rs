use super::Solution;
impl Solution {
//    pub fn is_palindrome(x: i32) -> bool {
//        if x < 0 {
//            false
//        } else {
//            let mut reversed_num = 0i32;
//            let mut num = x;
//            while num > 0 {
//                reversed_num = reversed_num*10 + num%10;
//                num = num/10;
//            }
//            reversed_num == x
//        }
//    }
    pub fn is_palindrome(x: i32) -> bool {
        if (x % 10 == 0 && x != 0) || x < 0{
            false
        } else {
            Solution::is_palindrome_helper(x,0)
        }
    }

    fn is_palindrome_helper(x: i32, y: i32) -> bool {
        let new_x = x/10;
        let new_y = y*10 + x%10;
        if new_y == new_x || new_y == x {
            true
        } else if new_x < new_y{
            false
        } else {
            Solution::is_palindrome_helper(new_x, new_y)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(10), false);
//        assert_eq!(Solution::is_palindrome(1534236469), false);
    }
}
