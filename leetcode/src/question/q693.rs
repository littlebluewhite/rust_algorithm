pub fn has_alternating_bits(n: i32) -> bool {
    let mut n = n as u32;
    let mut ans = true;
    let mut prev = n & 1;
    n >>= 1;
    while n > 0{
        let next = n & 1;
        if prev == next{
            return false
        }
        prev = next;
        n >>= 1;
    }
    ans
}