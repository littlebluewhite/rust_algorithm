pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    const STATES:usize = 1 << 10;
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![[false; STATES]; n];
    for i in 0..m{
        let row = &grid[i];
        for j in 0..n{
            let value = row[j] as usize;
            if i == 0 && j == 0{
                dp[0][value] = true;
                continue;
            }
            let mut next = [false; STATES];
            for x in 0..STATES {
                let up = if i > 0 {dp[j][x]} else {false};
                let left = if j > 0 {dp[j-1][x]} else {false};
                if up || left {
                    next[x^value] = true;
                }
            }
            dp[j] = next;
        }
    }
    dp[n-1].iter().position(|&x| x).unwrap() as i32
}