pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    let mut suffix = vec![0i32; n + 1];
    for i in (0..n).rev() {
        suffix[i] = suffix[i + 1] + piles[i];
    }
    let mut memo = vec![vec![-1i32; n + 1]; n + 1];
    fn dfs(i: usize, m: usize, suffix: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
        let n = suffix.len() - 1;
        if i >= n {
            return 0;
        }
        if i + 2 * m >= n {
            return suffix[i];
        }
        if memo[i][m] != -1 {
            return memo[i][m];
        }
        let mut ans = 0;
        for x in 1..=2 * m {
            ans = ans.max(suffix[i] - dfs(i + x, x.max(m), suffix, memo));
        }
        memo[i][m] = ans;
        ans
    }
    dfs(0, 1, &suffix, &mut memo)
}
