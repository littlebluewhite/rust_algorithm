pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    const MOD: i64 = 1_000_000_007;
    let n = board.len();
    let b: Vec<&[u8]> = board.iter().map(|s| s.as_bytes()).collect();
    let mut dp_sum = vec![vec![-1i64; n]; n];
    let mut dp_cnt = vec![vec![0i64; n]; n];
    dp_sum[n - 1][n - 1] = 0;
    dp_cnt[n - 1][n - 1] = 1;
    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if i == n - 1 && j == n - 1 {
                continue;
            }
            if b[i][j] == b'X' {
                continue;
            }
            let prev = [(i + 1, j), (i, j + 1), (i + 1, j + 1)];
            let mut best = -1;
            let mut cnt = 0;
            for pre in prev {
                if pre.0 == n || pre.1 == n || dp_sum[pre.0][pre.1] < 0 {
                    continue;
                }
                if dp_sum[pre.0][pre.1] > best {
                    best = dp_sum[pre.0][pre.1];
                    cnt = dp_cnt[pre.0][pre.1];
                } else if dp_sum[pre.0][pre.1] == best {
                    cnt += dp_cnt[pre.0][pre.1];
                }
            }
            if best < 0 {
                continue;
            }
            let value = if b[i][j] == b'E' {
                0
            } else {
                (b[i][j] - b'0') as i64
            };
            dp_sum[i][j] = best + value;
            dp_cnt[i][j] = cnt % MOD;
        }
    }
    if dp_cnt[0][0] == 0 {
        vec![0, 0]
    } else {
        vec![dp_sum[0][0] as i32, dp_cnt[0][0] as i32]
    }
}
