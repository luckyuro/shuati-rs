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

        for i in (2..=p.len()).step_by(2) {
            if p[i - 1] == STAR {
                m[i][0] = true;
            } else {
                break;
            }
        }

        for mpi in 1..=p.len() {
            let pat = p[mpi - 1];
            if pat == STAR {
                let rel_pat = p[mpi - 2];
                for msi in 1..=s.len() {
                    let c = s[msi - 1];
                    if rel_pat == c || rel_pat == DOT {
                        // situation:
                        // 1. -delete- match for multiple times  m[mpi - 1][msi - 1] not needed, covered by case 3
                        // 2. already matched once and only once
                        // 3. current line has matched and continue match
                        // 4. even matched but count zero (same pattern as former)
                        m[mpi][msi] = m[mpi - 1][msi] || m[mpi][msi - 1] || m[mpi - 2][msi];
                    } else {
                        // situation:
                        // 1. the count is zero
                        m[mpi][msi] = m[mpi - 2][msi];
                    }
                }
            } else {
                for msi in 1..=s.len() {
                    let c = s[msi - 1];
                    m[mpi][msi] = m[mpi - 1][msi - 1] && (pat == c || pat == DOT);
                }
            }
        }
        print!("{:?}", &m);
        m[p.len()][s.len()]
    }
}
mod test {
    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("aaa".to_string(), ".*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b*".to_string()),
            true
        );
    }
}
