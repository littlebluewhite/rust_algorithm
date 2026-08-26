pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
    let k = k as usize;
    let b = s.as_bytes();
    let ones: Vec<usize> = b
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == b'1' { Some(i) } else { None })
        .collect();
    if ones.len() < k {
        return "".to_string();
    }
    let mut best = None;
    for i in 0..=ones.len()-k{
        let start = ones[i];
        let end = ones[i+k-1];
        match best {
            None => best = Some((start, end)),
            Some((best_start, best_end)) => {
                if end - start < best_end - best_start{
                    best = Some((start, end));
                }else if end - start == best_end - best_start {
                    if b[start..=end] < b[best_start..=best_end]{
                        best = Some((start, end));
                    }
                }
            }
        }
    }
    let (start, end) = best.unwrap();
    s[start..=end].to_string()
}
