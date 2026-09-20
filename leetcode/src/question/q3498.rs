pub fn reverse_degree(s: String) -> i32 {
    let b = s.as_bytes();
    let mut ans = 0;
    for (i, &c) in b.iter().enumerate() {
        ans += (i as i32+1) * (26-(c-b'a') as i32)
    }
    ans
}