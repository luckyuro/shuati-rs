use crate::Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}