struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut even_count: usize = 0;
        let mut odd_count: usize = 0;
        let mut ret: Vec<i32> = vec![0; a.len()];

        for x in a.iter() {
            if x % 2 == 0 {
                ret[even_count * 2 ] = *x;
                even_count += 1;
            } else {
                ret[odd_count * 2 + 1] = *x;
                odd_count += 1;
            }
        }
        return ret;
    }
}