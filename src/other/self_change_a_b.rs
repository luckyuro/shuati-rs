pub fn change_a_b(s: String) -> usize {

    let len = s.len() ;
    let sc = s.as_bytes();


    let mut pa: usize = 0;
    let mut pb: usize = len - 1;
    let mut toa= 0usize;
    let mut tob = 0usize;
    let mut add_toa: bool = !(sc[pa] == 'a' as u8);
    let mut add_tob: bool = !(sc[pb] == 'b' as u8);
    let mut forward: bool = true;

    while pa < pb {
        if forward{
            if add_toa { toa += 1; add_toa=false; }
            pa = pa + 1;
            if sc[pa] == 'a' as u8 {
                continue;
            } else {
                forward = false;
                add_toa = true;
                continue;
            }
        } else {
            if add_tob { tob += 1; add_tob=false; }
            pb = pb - 1;
            if sc[pb] == 'b' as u8 {
                continue;
            } else {
                forward = true;
                add_tob = true;
                continue;
            }
        }
    }

    return if add_toa && add_tob {
        toa + tob + 1
    } else {
        toa + tob
    };


}

#[cfg(test)]
mod test {
    use super::change_a_b;

    #[test]
    fn test_change_a_b() {
        assert_eq!(change_a_b(String::from("aaabbb")), 0);
        assert_eq!(change_a_b(String::from("aaaaaaaab")), 0);
        assert_eq!(change_a_b(String::from("bbbabbb")), 1);
        assert_eq!(change_a_b(String::from("aaabbb")), 0);
        assert_eq!(change_a_b(String::from("aaabbb")), 0);
    }
}