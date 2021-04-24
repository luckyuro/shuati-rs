struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        if times.len() < (n-1) as usize { return -1 }

        let mut distance_from_k = vec![-1; (n+1) as usize];
        let mut point_visited = vec![false; (n+1) as usize];

        distance_from_k[k as usize] = 0;
        distance_from_k[0] = 0;

        point_visited[0] = true;

        loop {
            let mut node = 0;
            let mut min_time_not_visited = std::i32::MAX;

            for (i, v) in distance_from_k.iter().enumerate() {
                if !point_visited[i] && v > &-1 && v < &min_time_not_visited {
                    min_time_not_visited = *v;
                    node = i;
                }
            }
            if min_time_not_visited == std::i32::MAX { break; }

            point_visited[node] = true;


            for x in times.iter() {
                let (_source, _target, time) = (x[0], x[1], x[2]);
                let (source, target) = (_source as usize, _target as usize);
                if  node == source{
                    let new_time_to_target = min_time_not_visited + time;
                    if distance_from_k[target] == -1 || new_time_to_target < distance_from_k[target] {
                        distance_from_k[target] = new_time_to_target;
                    }
                }
            }
        }

        let x  = distance_from_k.iter().filter(|x|  **x>-1).collect::<Vec<&i32>>();
        if x.len() < distance_from_k.len() { -1 } else { **(x.iter().max().unwrap()) }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::network_delay_time(vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]],
                                                4,2), 2);
        assert_eq!(Solution::network_delay_time(vec![vec![1,2,1]],2,2), -1);
        assert_eq!(Solution::network_delay_time(vec![vec![1,2,1], vec![2,1,3]],2,2), 3);
        assert_eq!(Solution::network_delay_time(vec![vec![4,2,76],vec![1,3,79],vec![3,1,81],
                                                     vec![4,3,30],vec![2,1,47],vec![1,5,61],
                                                     vec![1,4,99],vec![3,4,68],vec![3,5,46],
                                                     vec![4,1,6],vec![5,4,7],vec![5,3,44],
                                                     vec![4,5,19],vec![2,3,13],vec![3,2,18],
                                                     vec![1,2,0],vec![5,1,25],vec![2,5,58],
                                                     vec![2,4,77],vec![5,2,74]], 5, 3), 59);
    }
}


//pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
//    if times.len() < (n-1) as usize { return -1 }
//    use std::collections::HashSet;
//
//    let mut distance_from_k = vec![-1; (n + 1) as usize];
//    let mut points_to_handle = Vec::new();
//    let mut points_handled = HashSet::new();
//
//    points_to_handle.push(k as usize);
//    points_handled.insert(k as usize);
//
//    distance_from_k[k as usize] = 0;
//    distance_from_k[0] = 0;
//
//
//    while !points_to_handle.is_empty() {
//        let mut point= points_to_handle[0];
//        let mut i_of_point = 0;
//        let mut min_w = distance_from_k[point];
//
//        for (i, v) in points_to_handle.iter().enumerate() {
//            if distance_from_k[*v] < min_w {
//                min_w = distance_from_k[*v];
//                i_of_point = i;
//                point = *v;
//            }
//        }
//        points_to_handle.remove(i_of_point);
//
//        for x in times.iter() {
//            let (source, target, time) = (x[0], x[1], x[2]);
//            let (source, target) = (source as usize, target as usize);
//            if source == point {
//                let new_time_to_target = distance_from_k[source] + time;
//                if distance_from_k[target] == -1 || new_time_to_target < distance_from_k[target] {
//                    distance_from_k[target] = new_time_to_target;
//                }
//                if !points_handled.contains(&target) {
//                    points_to_handle.push(target);
//                    points_handled.insert(target);
//                }
//            }
//        }
//    }
//    let x  = distance_from_k.iter().filter(|x|  **x>-1).collect::<Vec<&i32>>();
//    if x.len() < distance_from_k.len() { -1 } else { **(x.iter().max().unwrap()) }
//let mut ans = 0;
//for x in distance_from_k.iter() {
//if *x > -1 && *x > ans {
//ans = *x
//} else if *x < 0{
//return -1
//}
//}
//ans
//}