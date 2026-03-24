pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let row = grid.len();
    let col = grid[0].len();
    let mut max_dp = vec![vec![0; col]; row];
    let mut min_dp = vec![vec![0; col]; row];
    max_dp[0][0] = grid[0][0] as i64;
    min_dp[0][0] = grid[0][0] as i64;
    for r in 0..row {
        for c in 0..col {
            if r == 0 && c == 0 {
                continue;
            }
            let mut max = i64::MIN;
            let mut min = i64::MAX;
            if c > 0{
                max = max.max(max_dp[r][c - 1]);
                min = min.min(min_dp[r][c - 1]);
            }
            if r > 0{
                max = max.max(max_dp[r - 1][c]);
                min = min.min(min_dp[r - 1][c]);
            }
            let a =max * grid[r][c] as i64;
            let b =min * grid[r][c] as i64;
            max_dp[r][c] = a.max(b);
            min_dp[r][c] = a.min(b);
        }
    }
    let res = max_dp[row - 1][col - 1];
    if res < 0 {
        return -1;
    }
    (res % MOD) as i32
}
