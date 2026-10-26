pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let dir = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                continue;
            }
            let mut sum = 0;
            let mut stack = vec![(i, j)];
            while let Some((x, y)) = stack.pop() {
                sum = (sum + grid[x][y]) % k;
                grid[x][y] = 0;
                for (dx, dy) in dir {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx < 0
                        || nx >= m as i32
                        || ny < 0
                        || ny >= n as i32
                        || grid[nx as usize][ny as usize] == 0
                    {
                        continue;
                    }
                    stack.push((nx as usize, ny as usize));
                }
            }
            if sum % k == 0 {
                ans += 1;
            }
        }
    }
    ans
}
