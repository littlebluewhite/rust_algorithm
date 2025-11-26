use std::collections::VecDeque;

pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    const MOD: i64 = 1_000_000_007;

    let mut dp = vec![vec![0; k as usize]; n];
    dp[0][(grid[0][0] % k) as usize] = 1;

    for i in 0..m {
        let mut new_dp = vec![vec![0; k as usize]; n];
        for j in 0..n {
            let val = grid[i][j] % k;
            if i > 0 {
                for r in 0..k {
                    let nr = ((r + val) % k) as usize;
                    new_dp[j][nr] = (new_dp[j][nr] + dp[j][r as usize]) % MOD;
                }
            }
            if j > 0 {
                for r in 0..k {
                    let nr = ((r + val) % k) as usize;
                    new_dp[j][nr] = (new_dp[j][nr] + new_dp[j - 1][r as usize]) % MOD;
                }
            }
            if i == 0 && j == 0 {
                new_dp[j][val as usize] = 1;
            }
        }
        dp = new_dp;
    }
    dp[n - 1][0] as i32
}
