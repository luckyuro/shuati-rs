use super::Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();

        let mut remain_gas = 0;
        let mut owed_gas = 0;
        let mut start = 0;
        for i in 0..len {
            remain_gas = remain_gas + gas[i] - cost[i];
            if remain_gas < 0 {
                owed_gas += remain_gas;
                remain_gas = 0;
                start = i + 1;
            }
        }

        if remain_gas + owed_gas >= 0 {
            start as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test{

    use super::Solution;


    #[test]
    fn test_three_sum_multi() {
        assert_eq!(Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
        assert_eq!(Solution::can_complete_circuit(vec![3,3,4], vec![3,4,4]), -1);
        assert_eq!(Solution::can_complete_circuit(vec![5,1,2,3,4], vec![4,4,1,5,1]), 4);
    }
}


//pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//    let len = gas.len();
//    for start_point in 0..len {
//        if gas[start_point] < cost[start_point] {
//            continue;
//        } else {
//            let mut p = start_point;
//            let mut remain_gas = 0;
//            let mut succeed = true;
//
//            loop {
//                remain_gas = remain_gas + gas[p] - cost[p];
//
//                p = (p+1) % len;
//
//                if remain_gas < 0 {
//                    succeed = false;
//                    break;
//                } else if p == start_point {
//                    break;
//                }
//            }
//
//            if succeed {
//                return start_point as i32;
//            }
//        }
//    }
//    -1
//}