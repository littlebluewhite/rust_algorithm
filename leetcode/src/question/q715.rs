pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let n = s1_bytes.len();
    let m = s2_bytes.len();
    let mut dp = vec![vec![0i32; m + 1]; n + 1];
    for i in 0..n {
        dp[i + 1][0] = dp[i][0] + s1_bytes[i] as i32
    }
    for j in 0..m {
        dp[0][j + 1] = dp[0][j] + s2_bytes[j] as i32
    }
    for i in 0..n {
        for j in 0..m {
            if s1_bytes[i] == s2_bytes[j] {
                dp[i + 1][j + 1] = dp[i][j]
            } else {
                let del_s1 = dp[i][j + 1] + s1_bytes[i] as i32;
                let del_s2 = dp[i + 1][j] + s2_bytes[j] as i32;
                dp[i + 1][j + 1] = del_s1.min(del_s2);
            }
        }
    }
    dp[n][m]
}
