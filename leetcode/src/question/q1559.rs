pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();

    let mut visited = vec![vec![false;m];n];
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for i in 0..n{
        for j in 0..m{
            let target = grid[i][j];
            if visited[i][j]{
                continue;
            }
            let mut stack: Vec<(usize, usize, Option<(usize, usize)>)> = vec![(i, j, None)];
            while let Some((sr, sc, parent)) = stack.pop(){
                for (dr, dc) in dirs{
                    let (nr, nc) = (dr + sr as i32, dc + sc as i32);
                    if nr < 0 || nr >= n as i32 || nc < 0 || nc >= m as i32 {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] != target{
                        continue;
                    }
                    if parent == Some((nr, nc)){
                        continue;
                    }
                    if visited[nr][nc]{
                        return true;
                    }
                    visited[nr][nc] = true;
                    stack.push((nr, nc, Some((sr, sc))));
                }
            }
        }
    }
    false
}