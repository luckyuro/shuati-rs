use super::Solution;
#[allow(non_snake_case, unused)]
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let solution_0_ = {
            let M = ["", "M", "MM", "MMM"];
            let C = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
            let X = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
            let I = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
            format!(
                "{}{}{}{}",
                M[(num / 1000) as usize],
                C[(num % 1000 / 100) as usize],
                X[(num % 100 / 10) as usize],
                I[(num % 10) as usize]
            )
        };
        let roman_value = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let int_value = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let mut ans = String::new();
        let mut num = num;
        for (i, v) in int_value.iter().enumerate() {
            while v <= &num {
                num -= v;
                ans.push_str(roman_value[i])
            }
        }
        ans
    }
}
