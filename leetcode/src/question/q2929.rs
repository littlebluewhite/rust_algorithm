pub fn distribute_candies(n: i32, limit: i32) -> i64 {
    let n = n as i64;
    let limit = limit as i64;
    c2(n + 2) - c2(n - limit + 1) * 3 + c2(n - limit * 2)*3 - c2(n - limit * 3 - 1)
}
fn c2(k: i64) -> i64 {
    if k < 2 {
        return 0;
    }
    k * (k - 1) / 2
}
