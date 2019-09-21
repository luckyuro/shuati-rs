struct Solution;

impl Solution {
    pub fn partition_disjoint(a: Vec<i32>) -> i32 {
        let len = a.len();
        let a0 = a[0];
        return partition_disjoint_rec(a, len, a0, a0, 1, 1);
    }
}

fn partition_disjoint_rec(v: Vec<i32>, len: usize, l_max: i32, max: i32, p: usize, count: usize) -> i32 {
    if p + 1 >= len {
        return count as i32
    };

    let new_max = if v[p] > max { v[p] } else { max };
    if v[p] < l_max {
        partition_disjoint_rec(v, len, new_max, new_max, p+1, p+1)
    } else {
        partition_disjoint_rec(v, len, l_max, new_max, p+1, count)
    }

}