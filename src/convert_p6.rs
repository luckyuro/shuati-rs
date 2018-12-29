pub fn convert_p6(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s
    }

    let len = s.len() ;
    let sc = s.as_bytes();
    let mut ans: Vec<u8> = Vec::with_capacity(len);
    let mut step1: usize;
    let mut step2: usize;
    let rows = num_rows as usize;

    for n in 0usize..rows{
        if rows < n + 1{
            continue
        }
        step1 = (rows - n - 1) * 2;
        step2 = n * 2;

        let mut i = n;
        if i < len {
            ans.push(sc[i]);
        }
        loop {
            i = i + step1;
            if i >= len {
                break;
            }
            if step1 != 0 {
                ans.push(sc[i]);
            }
            i = i + step2;
            if i >= len {
                break;
            }
            if step2 != 0 {
                ans.push(sc[i]);
            }
        }
    }
    let ans = String::from_utf8(ans);
    ans.unwrap()

}

#[cfg(test)]
mod test {
    use super::convert_p6;

    #[test]
    fn test_convert_p6() {
        assert_eq!(convert_p6(String::from("PAYPALISHIRING"), 3), String::from("PAHNAPLSIIGYIR"));
    }
}