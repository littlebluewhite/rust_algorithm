pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    let mut total = 0i64;
    for i in 0..n {
        for j in 0..m {
            total += grid[i][j] as i64;
        }
    }
    if total % 2 != 0 {
        return false;
    }
    let mut row_prefix = 0i64;
    for i in 0..n - 1 {
        for j in 0..m {
            row_prefix += grid[i][j] as i64;
        }
        if row_prefix * 2 == total {
            return true;
        } else if row_prefix * 2 > total {
            break;
        }
    }
    let mut col_prefix = 0i64;
    for j in 0..m - 1 {
        for i in 0..n {
            col_prefix += grid[i][j] as i64;
        }
        if col_prefix * 2 == total {
            return true;
        } else if col_prefix * 2 > total {
            break;
        }
    }
    false
}
