pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
    const NEG: i32 = -1_000_000_000;
    let n = coins.len();
    let m = coins[0].len();
    let mut dp = vec![vec![vec![NEG; 3]; m]; n];
    dp[0][0][0] = coins[0][0];
    if coins[0][0] < 0 {
        dp[0][0][1] = 0;
    }
    for i in 0..n {
        for j in 0..m {
            if i == 0 && j == 0 {
                continue;
            }
            let val = coins[i][j];
            for used in 0..3 {
                let mut best_pre = NEG;
                if i > 0 {
                    best_pre = best_pre.max(dp[i - 1][j][used]);
                }
                if j > 0 {
                    best_pre = best_pre.max(dp[i][j - 1][used]);
                }
                if best_pre != NEG {
                    dp[i][j][used] = best_pre + val;
                }
                if val < 0 && used > 0 {
                    let mut best_pre_n = NEG;
                    if i > 0 {
                        best_pre_n = best_pre_n.max(dp[i - 1][j][used - 1]);
                    }
                    if j > 0 {
                        best_pre_n = best_pre_n.max(dp[i][j - 1][used - 1]);
                    }
                    if best_pre_n != NEG {
                        dp[i][j][used] = dp[i][j][used].max(best_pre_n);
                    }
                }
            }
        }
    }
    *dp[n - 1][m - 1].iter().max().unwrap()
}
