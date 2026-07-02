use std::collections::VecDeque;

pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut max_health = vec![vec![0;m as usize];n as usize];
    if health - grid[0][0] < 1 {
        return false;
    }
    q.push_back((health - grid[0][0], 0, 0));
    while let Some((health, r, c)) = q.pop_front() {
        if r == n - 1 && c == m - 1 {
            return true;
        }
        for dir in dirs {
            let (nr, nc) = (r + dir.0, c + dir.1);
            if nr < 0 || nr >= n || nc < 0 || nc >= m {
                continue;
            }
            let loss_health = grid[nr as usize][nc as usize];
            if health - loss_health < 1 || max_health[nr as usize][nc as usize] >= health - loss_health {
                continue;
            }
            max_health[nr as usize][nc as usize] = health - loss_health;
            if grid[nr as usize][nc as usize] == 1 {
                q.push_back((health - grid[nr as usize][nc as usize], nr, nc));
            } else {
                q.push_front((health - grid[nr as usize][nc as usize], nr, nc));
            }
        }
    }
    false
}
