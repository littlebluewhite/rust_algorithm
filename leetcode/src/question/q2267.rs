pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let path_len = m + n - 1;
    if path_len % 2 != 0|| grid[0][0] != '(' || grid[m-1][n-1] != ')'{
        return false;
    }
    let max_balance = path_len / 2;
    let mut dp: Vec<Vec<bool>> = vec![vec![false;max_balance+1]; n];
    for r in 0..m {
        for c in 0..n {
            if r == 0 && c == 0{
                dp[0][1] = true;
                continue;
            };
            let mut next = vec![false;max_balance+1];
            let value = if grid[r][c] == '(' {1} else {-1};
            for balance in 0..=max_balance {
                let pre_balance = balance as i32 -value;
                if pre_balance < 0 || pre_balance > max_balance as i32{
                    continue;
                }
                if r > 0{
                    next[balance] |= dp[c][pre_balance as usize];
                }
                if c > 0{
                    next[balance] |= dp[c - 1][pre_balance as usize];
                }
            }
            dp[c] = next;
        }
    }
    dp[n - 1][0]
}