pub fn reverse_bits(x: i32) -> i32 {
    let mut x = x as u32;
    let mut ans = 0u32;
    for _ in 0..32 {
        ans = (ans << 1) | (x & 1);
        x >>= 1;
    }
    ans as i32
}