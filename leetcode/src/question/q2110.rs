pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let mut ans = 1i64;
    let mut left = 0;
    for i in 1..prices.len() {
        if prices[i-1] - prices[i] != 1 {
            left = i;
        }
        ans += (i - left) as i64 + 1;
    }
    ans
}
