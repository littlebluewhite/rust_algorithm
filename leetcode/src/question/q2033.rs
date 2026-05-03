pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let base = grid[0][0];
    let mut values = Vec::with_capacity(m*n);
    for i in 0..n {
        for j in 0..m {
            let value = grid[i][j];
            if (value-base) % x != 0 {
                return -1;
            }
            values.push(value);
        }
    }
    values.sort_unstable();
    let mut ans = 0;
    let target = values[(values.len())/2];
    for value in values{
        ans += ((value-target).abs())/x;
    }
    ans
}