pub fn num_distinct(s: String, t: String) -> i32 {
    let t_bytes = t.as_bytes();
    let n = t_bytes.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for c in s.bytes() {
        for j in (1..=n).rev() {
            if c == t_bytes[j-1] {
                dp[j] += dp[j-1];
            }
        }
    }
    dp[n]
}