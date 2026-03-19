pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut x_count = vec![0;m];
    let mut y_count = vec![0;m];
    let mut ans = 0;
    for i in 0..n{
        let mut x = 0;
        let mut y = 0;
        for j in 0..m{
            if grid[i][j] == 'X'{
                x += 1;
            }else if grid[i][j] == 'Y'{
                y += 1;
            }
            x_count[j] += x;
            y_count[j] += y;
            if x_count[j] == y_count[j] && x_count[j] > 0 {
                ans += 1;
            }
        }

    }
    ans
}