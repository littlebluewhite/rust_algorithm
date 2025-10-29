pub fn smallest_number(n: i32) -> i32 {
    // n >= 1 (依題目約束)
    let n = n as u32;
    let bits = 32 - n.leading_zeros(); // n 的有效位數
    ((1u32 << bits) - 1) as i32        // 2^bits - 1
}