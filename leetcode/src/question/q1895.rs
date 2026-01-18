pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
    let row = grid.len();
    let column = grid[0].len();
    let mut row_prefix = vec![vec![0; column + 1]; row];
    let mut column_prefix = vec![vec![0; column]; row + 1];
    let mut diagonal_left = vec![vec![0; column + 1]; row + 1];
    let mut diagonal_right = vec![vec![0; column + 1]; row + 1];
    for r in 0..row {
        for c in 0..column {
            let val = grid[r][c];
            row_prefix[r][c + 1] = row_prefix[r][c] + val;
            column_prefix[r + 1][c] = column_prefix[r][c] + val;
            diagonal_left[r + 1][c + 1] = diagonal_left[r][c] + val;
        }
        for c in (0..column).rev() {
            let val = grid[r][c];
            diagonal_right[r + 1][c] = diagonal_right[r][c + 1] + val;
        }
    }
    println!("row_prefix: {:?}", row_prefix);
    println!("column_prefix: {:?}", column_prefix);
    println!("diagonal_left: {:?}", diagonal_left);
    println!("diagonal_right: {:?}", diagonal_right);
    println!("row: {}", row);
    println!("column: {}", column);
    let mut max = row.min(column);
    while max > 1 {
        for r in 0..=row - max {
            'a: for c in 0..=column - max {
                let d_r = diagonal_right[r + max][c] - diagonal_right[r][c + max];
                for i in r..r + max {
                    if row_prefix[i][c + max] - row_prefix[i][c] != d_r {
                        continue 'a;
                    }
                }
                for j in c..c + max {
                    if column_prefix[r + max][j] - column_prefix[r][j] != d_r {
                        continue 'a;
                    }
                }
                let d_l = diagonal_left[r + max][c + max] - diagonal_left[r][c];
                if d_l != d_r {
                    continue 'a;
                }
                return max as i32;
            }
        }
        max -= 1;
    }
    max as i32
}
