pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let dir = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = 0;
    for i in 0..m{
        for j in 0..n{
            if grid[i][j] == 1{
                continue;
            }
            let mut stack: Vec<(usize, usize)> = vec![];
            grid[i][j] = 1;
            stack.push((i, j));
            let mut closed = true;
            while let Some((x, y)) = stack.pop() {
                if x == 0 || x == m - 1 || y == 0 || y == n - 1 {
                    closed = false;
                }
                for (dx, dy) in dir{
                    let (nx, ny) = (x as i32+ dx, y as i32+ dy);
                    if nx < 0 || nx >= m as i32|| ny < 0 || ny >= n as i32|| grid[nx as usize][ny as usize] == 1{
                        continue;
                    }
                    grid[nx as usize][ny as usize] = 1;
                    stack.push((nx as usize, ny as usize));
                }
            }
            if closed{
                ans += 1;
            }
        }
    }
    ans
}