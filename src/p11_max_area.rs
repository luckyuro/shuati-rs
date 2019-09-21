struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        return max_area_rec(height, 0, len - 1, 0)
    }
}

fn max_area_rec(height: Vec<i32>, start: usize, end: usize, max_g: i32) -> i32 {
    if start == end {
        return max_g
    };

    let is_start_smaller = height[start] < height[end];
    let max: i32 = if is_start_smaller {
        height[start] * (end - start) as i32
    } else {
        (end - start) as i32 * height[end]
    };

    let new_max = if max > max_g { max } else { max_g };

    if is_start_smaller{
        return max_area_rec(height, start + 1, end, new_max)
    } else {
        return max_area_rec(height, start, end - 1, new_max)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_is_match() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}