pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k % 5 == 0 || k % 2 == 0 {
        return -1;
    }
    let k = k as i64;
    let mut remainder = 0i64;
    for n in 1..=k as i32{
        remainder = (remainder * 10 + 1) % k;
        if remainder % k == 0 {
            return n;
        }
    }
    -1
}
