use std::cmp::min;
use std::collections::HashSet;
use super::Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![0; 366];
        let mut travel_days: HashSet<usize> = HashSet::new();

        for day in days.iter() { travel_days.insert(*day as usize); };

        for i in 1..366 {
            if travel_days.contains(&i ) {
                dp[i] = three_min(dp[i - 1] + costs[0],
                                  dp[i.saturating_sub(7)] + costs[1],
                                  dp[i.saturating_sub(30)] + costs[2]);
            } else {
                dp[i] = dp[i-1]
            }
        }
        dp[365]
    }
}

fn three_min(num1: i32, num2: i32, num3: i32) -> i32 {
    if num1 <= num2 && num1 <= num3 {
        num1
    } else if num2 <= num1 && num2 <= num3 {
        num2
    } else {
        num3
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::p983_mincost_tickets::three_min;

    #[test]
    fn test_combination() {
        assert_eq!(three_min(7, 3, 1), 1);
        assert_eq!(three_min(49, 6, 5), 5);
    }

    #[test]
    fn test_three_sum_multi() {
        assert_eq!(Solution::mincost_tickets(vec![1,4,6,7,8,20], vec![2,7,15]), 11);
        assert_eq!(Solution::mincost_tickets(vec![1,2,3,4,5,6,7,8,9,10,30,31], vec![2,7,15]), 17);
        assert_eq!(Solution::mincost_tickets(vec![1,4,6,7,8,20], vec![7,2,15]), 6);
        assert_eq!(Solution::mincost_tickets(vec![1,2,3,4,6,8,9,10,13,14,16,17,19,21,24,26,27,28,29], vec![3,14,50]), 50);
        assert_eq!(Solution::mincost_tickets(vec![1,2,4,5,6,9,10,12,14,15,18,20,21,22,23,24,25,26,28], vec![3,13,57]), 45);
    }
}