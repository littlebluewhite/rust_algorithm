pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut row_count = vec![0; m];
    let mut col_count = vec![0; n];
    let mut point: Vec<(usize, usize)> = Vec::with_capacity(m * n);
    let mut ans = 0;
    for r in 0..m {
        for c in 0..n {
            if grid[r][c] == 1 {
                point.push((r, c));
                row_count[r] += 1;
                col_count[c] += 1;
            }
        }
    }
    for (r, c) in point {
        if row_count[r] == 1 && col_count[c] == 1 {
            continue;
        }
        ans += 1;
    }
    ans
}
