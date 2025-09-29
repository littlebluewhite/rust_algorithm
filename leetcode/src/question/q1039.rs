pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut dp = vec![vec![0i32; n]; n]; // dp[i][j]：區間 i..=j 的最小分數
    println!("{:?}", dp);

    // len 是 j - i（區間跨度）。至少要 2 才能構成 3 個點（能畫出三角形）
    for len in 2..n {
        for i in 0..(n - len) {
            let j = i + len;
            let mut best = i32::MAX; // 先放一個很大的值，等等取 min

            // 嘗試把最後一個三角形選成 (i, k, j)
            for k in (i + 1)..j {
                // 子問題都比當前區間短，所以這裡已經算好
                let cand = dp[i][k]
                    + dp[k][j]
                    + values[i] * values[k] * values[j];

                if cand < best {
                    best = cand;
                }
            }
            dp[i][j] = best;
        }
    }

    dp[0][n - 1]
}