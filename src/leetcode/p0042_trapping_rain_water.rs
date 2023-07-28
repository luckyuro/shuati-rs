use super::Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let last = height.len() - 1;
        let (mut li, mut ri) = (0usize, last);

        let mut all = 0i32;
        let (mut lv, mut rv) = (height[li], height[ri]);
        while li != ri {
            if lv < rv {
                li += 1;
                if height[li] <= lv {
                    all += (lv - height[li]);
                } else {
                    lv = height[li]
                }
            } else {
                ri -= 1;
                if height[ri] <= rv {
                    all += (rv - height[ri])
                } else {
                    rv = height[ri];
                }
            }
        }
        all
    }
}
