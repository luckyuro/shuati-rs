struct Solution;
use std::collections::HashMap;
use std::cmp::Ordering;

impl Solution {
    pub fn three_sum_multi(a: Vec<i32>, target: i32) -> i32 {
        let mut num_times: HashMap<i32, usize> = HashMap::new();
        let mut num_once: Vec<i32> = Vec::new();

        for num in a.iter() {
            if num_times.contains_key(num) {
                *num_times.get_mut(num).unwrap() += 1;
            } else {
                num_once.push(*num);
                num_times.insert(*num, 1);
            }
        }

        let mut answer = 0;
        let limit: usize = 1000000007;
        num_once.sort();

        for p_anchor in 0..num_once.len() {
            let target = target - num_once[p_anchor];

            let mut p_start: usize = if num_times[&num_once[p_anchor]]> 1 { p_anchor } else { p_anchor+1 };
            let mut p_end = num_once.len() - 1;

            while p_start <= p_end {
                match (num_once[p_start] + num_once[p_end]).cmp(&target) {
                    Ordering::Less => { p_start += 1; },
                    Ordering::Greater => { p_end -= 1; },
                    Ordering::Equal => {
                        if p_start == p_end && p_end == p_anchor {
                            answer += match combination(num_times[&num_once[p_anchor]], 3) {
                                Err(_) => 0,
                                Ok(r) => r
                            };
                        } else if p_start == p_anchor {
                            answer += combination(num_times[&num_once[p_start]], 2)
                                .unwrap() * num_times[&num_once[p_end]];
                        } else if p_start == p_end {
                            answer += match combination(num_times[&num_once[p_start]], 2) {
                                Err(_) => 0,
                                Ok(r) => r * num_times[&num_once[p_anchor]]
                            };
                        } else {
                            answer += num_times[&num_once[p_anchor]] * num_times[&num_once[p_start]] * num_times[&num_once[p_end]];
                        };
                        answer %= limit;
                        p_start += 1;
                        if p_end == 0 {
                            break;
                        } else {
                            p_end -= 1;
                        }
                    }
                }
            }
        }

        answer as i32
    }
}

fn combination(set_size: usize, select_size: usize) -> Result<usize, i32> {
    if select_size > set_size {
        Err(-1)
    } else {
        Ok(combination_helper(set_size, select_size, 1, 1))
    }
}

fn combination_helper(upper: usize, lower: usize, upper_product: usize, lower_product: usize) -> usize {
    if lower == 1 {
        upper * upper_product / lower_product
    } else {
        combination_helper(upper-1, lower-1, upper_product*upper, lower_product*lower)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::p923_three_sum_multi::combination;

    #[test]
    fn test_combination() {
        assert_eq!(combination(7, 3), Ok(35));
        assert_eq!(combination(49, 6), Ok(13983816));
        assert_eq!(combination(2, 2), Ok(1));
        assert_eq!(combination(1, 2), Err(-1));
    }

    #[test]
    fn test_three_sum_multi() {
        assert_eq!(Solution::three_sum_multi(vec![1,1,2,2,3,3,4,4,5,5], 8), 20);
        assert_eq!(Solution::three_sum_multi(vec![1,1,1,1,1,1,1], 3), 35);
        assert_eq!(Solution::three_sum_multi(vec![1,2,3,3,1], 5), 2);
    }
}