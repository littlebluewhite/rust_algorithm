const MOD: i64 = 1000000007;
pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut down = vec![0i64; n];
    down[0] = 1;
    for i in 0..m {
        let mut right = 0i64;
        for j in 0..n {
            let from_above = down[j];
            if grid[i][j] == 1 {
                down[j] = right;
                right = from_above;
            } else {
                let way = (from_above + right)%MOD;
                down[j] = way;
                right = way;
            }
        }
    }
    down[n - 1] as i32
}
