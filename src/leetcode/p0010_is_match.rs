// to check
use super::Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut m = vec![vec![false; s.len() + 1]; p.len() + 1];
        m[0][0] = true;

        const STAR: u8 = '*' as u8;
        const DOT: u8 = '.' as u8;

        for (pi, pat) in p.iter().enumerate() {
            if pat == &STAR {
                for (si, c) in s.iter().enumerate() {
                    if si == 0 {
                        continue;
                    }
                    let pat = &p[pi - 1];
                    if pat == c || pat == &DOT {
                        m[pi + 1][si + 1] = m[pi][si];
                    } else {
                        m[pi + 1][si + 1] = m[pi][si + 1]
                            || m[pi + 1][si]
                            || m[pi - 1][si + 1]
                            || m[pi - 1][si - 1];
                    }
                }
                continue;
            }
            for (si, c) in s.iter().enumerate() {
                m[pi + 1][si + 1] = m[pi][si] && (pat == c || pat == &DOT);
            }
        }
        print!("{:?}", &m);
        m[p.len()][s.len()]
    }
}
mod test {
    use super::Solution;
    #[test]
    fn test_is_match() {
        // assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        // assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        //  assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b*".to_string()),
            true
        );
    }
}
