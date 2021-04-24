pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<u8, usize> = HashMap::with_capacity(26);
    let mut len: usize = 0;
    let mut max: usize = 0;
    let mut start: usize = 0;
    for (i, c) in s.chars().enumerate() {
        let v = map.entry(c as u8).or_insert(i);
        if *v != i && *v >= start {
            len = i - *v;
            start = *v;
            map.insert(c as u8, i);
        } else if *v != i{
            map.insert(c as u8, i);
            len += 1;
        } else {
            len += 1;
        }
        max = if max > len  {max} else {len};
    }
    return max as i32;
}

