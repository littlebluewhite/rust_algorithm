pub fn has_all_codes(s: String, k: i32) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    let need = 1 << k;
    let mut seen = vec![false; need];
    let full = need - 1;
    let mut count = 0;
    if n < need + k as usize -1{
        return false;
    }
    let mut mask = 0usize;
    for i in 0..n{
        let bit = s[i] - b'0';
        mask = ((mask << 1) & full) | bit as usize;
        if i >= k as usize -1 && !seen[mask]{
            seen[mask] = true;
            count += 1;
            if count == need{
                return true;
            }
        }
    }
    false
}