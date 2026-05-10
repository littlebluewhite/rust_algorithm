pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let n = grid.len();
    let m = grid[0].len();
    let layers = n.min(m) / 2;
    for layer in 0..layers {
        let top = layer;
        let left = layer;
        let bottom = n - layer - 1;
        let right = m - layer - 1;
        let mut coords = Vec::new();
        let mut values = Vec::new();
        for r in top..=bottom {
            coords.push((r, left));
            values.push(grid[r][left]);
        }
        for c in left + 1..=right {
            coords.push((bottom, c));
            values.push(grid[bottom][c]);
        }
        for r in (top..bottom).rev() {
            coords.push((r, right));
            values.push(grid[r][right]);
        }
        for c in (left + 1..right).rev() {
            coords.push((top, c));
            values.push(grid[top][c]);
        }
        let shift = k as usize % coords.len();
        for i in 0..coords.len() {
            let new_index = (shift + i) % coords.len();
            let (nr, nc) = coords[new_index];
            grid[nr][nc] = values[i];
        }
    }
    grid
}
