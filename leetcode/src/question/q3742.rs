pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const MAP: [usize; 3] = [0, 1, 1];
    let n = grid.len();
    let m = grid[0].len();
    let k = k as usize;
    let mut dp = vec![vec![vec![-1; k + 1]; m]; n];
    for i in 0..n {
        for j in 0..m {
            let score = grid[i][j];
            let cost = MAP[score as usize];
            let mut curr = vec![-1; k + 1];
            if i == 0 && j == 0 {
                if cost <= k {
                    curr[cost] = score;
                }
            }
            for c in cost..=k {
                let prev_cost = c - cost;
                let mut best = -1;
                if i > 0 {
                    best = best.max(dp[i - 1][j][prev_cost]);
                }
                if j > 0 {
                    best = best.max(dp[i][j - 1][prev_cost]);
                }
                if best != -1 {
                    curr[c] = best + score;
                }
            }
            dp[i][j] = curr;
        }
    }
    *dp[n - 1][m - 1].iter().max().unwrap()
}
