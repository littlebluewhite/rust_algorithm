pub fn bitwise_complement(n: i32) -> i32 {
    if n == 0{
        return 1
    }
    let mut m = 1;
    while m < n{
        m = (m << 1) | 1;
    }
    m ^ n
}