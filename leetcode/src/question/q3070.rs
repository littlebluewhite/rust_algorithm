pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut ans = 0;
    for i in 0..n {
        for j in 1..m {
            grid[i][j] += grid[i][j - 1];
        }
    }
    for j in 0..m {
        let mut sum = 0;
        for i in 0..n {
            sum += grid[i][j];
            if sum <= k {
                ans += 1;
            } else {
                break;
            }
        }
    }
    ans
}

// 0 0 0 0 0
// 0 1 2 3 4
// 0 2 4 6 8
// 0 3 6 9 12
// 0 4 8 12 16

// 0 0 0 0 0
// 0 1 3 6 10
// 0 3 9
