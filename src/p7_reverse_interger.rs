use crate::Solution;
impl Solution {
    fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let a = if x > 0 {x} else {-x};
        let flag = x/ a;
        let ans = Solution::reverse_nature(a, 0, false);
        return flag*ans;
    }

    fn reverse_nature(origin: i32, result: i32, end: bool) -> i32 {
        if end {
            return result
        } else {
            let new_result = result.checked_mul(10);
            if new_result == None {
                return 0;
            }
            let new_result = new_result.unwrap().checked_add(origin%10);
            if new_result == None {
                return 0;
            }
            let origin  = origin / 10;
            return Solution::reverse_nature(origin, new_result.unwrap(), origin==0);
        }
    }

}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}
