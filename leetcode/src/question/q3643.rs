pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
    let (x, y, k) = (x as usize, y as usize, k as usize);
    let mut top = x;
    let mut bottom = x + k - 1;
    while top < bottom {
        for col in y..(y + k) {
            let tmp = grid[top][col];
            grid[top][col] = grid[bottom][col];
            grid[bottom][col] = tmp;
        }
        top += 1;
        bottom -= 1;
    }
    grid
}