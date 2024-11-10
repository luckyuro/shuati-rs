fn minimum_number_move_to_sort(array: &[i32]) -> i32 {
    let sorted = {
        let mut v = array.to_vec();
        v.sort();
        v
    };

    fn minimum_sorted(origin: &[i32], sorted: &[i32], po: usize, ps: usize) -> i32 {
        if origin == sorted || po >= origin.len() || ps >= sorted.len() {
            // exit
            0
        } else if origin[po] < sorted[po] {
            1 + minimum_sorted(origin, sorted, po + 1, ps + 1)
        } else {
            std::cmp::min(
                minimum_sorted(origin, sorted, po + 1, ps),
                minimum_sorted(origin, sorted, po, ps + 1),
            )
        }
    }
    minimum_sorted(array, &sorted, 0, 0)
}

fn minimum_number_move_to_sort_dp(array: &[i32]) -> i32 {
    let sorted = {
        let mut v = array.to_vec();
        v.sort();
        v
    };
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_minimum() {
        let test1 = vec![4, 7, 2, 3, 9];
        assert_eq!(minimum_number_move_to_sort(&test1), 2);
    }
}
