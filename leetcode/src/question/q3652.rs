pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let half = k/2;
    let n = prices.len();
    let mut prefix_s: Vec<i64> = vec![0;n+1];
    let mut prefix_p: Vec<i64> = vec![0;n+1];
    for i in 0..n {
        prefix_s[i+1] = prefix_s[i] + (prices[i] * strategy[i]) as i64;
        prefix_p[i+1] = prefix_p[i] + prices[i] as i64;
    }
    let mut best = 0i64;
    for i in 0..=n-k{
        let s = prefix_s[i+k]-prefix_s[i];
        let p = prefix_p[i+k]-prefix_p[i+half];
        if p-s>best{
            best = p-s;
        }
    }
    prefix_s[n] + best
}