pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let all = nums1.len() + nums2.len();

    if all % 2 == 0 {
        let r1: f64 = find_kth(&nums1, &nums2, all/2) as f64;
        let r2: f64 = find_kth(&nums1, &nums2, all/2 + 1) as f64;
        (r1 + r2)/2.0
    } else {
        find_kth(&nums1, &nums2, all/2 + 1) as f64
    }
}

fn find_kth(nums1: &[i32], nums2: &[i32], k: usize) -> i32{
    let l1 = nums1.len();
    let l2 = nums2.len();
    if l1 > l2 {
        find_kth(nums2, nums1, k)
    } else if l1 == 0{
        nums2[k-1]
    } else if k == 1{
        std::cmp::min(nums1[0], nums2[0])
    } else {
        let p1: usize = std::cmp::min(k/2, l1);
        let p2: usize = k - p1;

        if nums1[p1-1] < nums2[p2-1]{
            find_kth(&nums1[p1..], nums2, k-p1)
        } else if nums1[p1-1] > nums2[p2-1]{
            find_kth(nums1, &nums2[p2..], k-p2)
        } else{
            //nums1[p1-1] == nums2[p2-1]
            nums1[p1-1]
        }
    }
}


#[cfg(test)]
mod test {
    use super::find_median_sorted_arrays;
    use super::find_kth;

    #[test]
    fn test_find_kth() {
        assert_eq!(find_kth(&[2, 7, 11, 15], &[2, 7, 11, 15], 4), 7);
        assert_eq!(find_kth(&[3, 4], &[1, 2], 2), 2);
        assert_eq!(find_kth(&[3, 4], &[1, 2], 3), 3);
    }

    #[test]
    fn test_find_median() {
        assert_eq!(find_median_sorted_arrays(vec![2, 7, 11, 15], vec![2, 7, 11, 15]), 9.0);
        assert_eq!(find_median_sorted_arrays(vec![1,3], vec![2]), 2f64);
        assert_eq!(find_median_sorted_arrays(vec![3,4], vec![1,2]), 2.5f64);


    }
}