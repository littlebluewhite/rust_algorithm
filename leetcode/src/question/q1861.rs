pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = box_grid.len();
    let m = box_grid[0].len();
    let mut res = vec![vec!['.'; n]; m];
    for i in 0..n {
        let mut write = m as isize - 1;
        for j in (0..m).rev() {
            match box_grid[i][j] {
                '*' => {
                    res[j][n - i - 1] = '*';
                    write = j as isize - 1;
                }
                '#' => {
                    res[write as usize][n - i - 1] = '#';
                    write -= 1;
                }
                _ => {}
            }
        }
    }
    res
}
