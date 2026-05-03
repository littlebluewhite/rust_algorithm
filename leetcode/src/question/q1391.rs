use std::collections::VecDeque;

pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    const UP: u8 = 1 << 0;
    const DOWN: u8 = 1 << 1;
    const LEFT: u8 = 1 << 2;
    const RIGHT: u8 = 1 << 3;

    const MASK: [u8; 7] = [
        0,
        LEFT | RIGHT,
        UP | DOWN,
        LEFT | DOWN,
        RIGHT | DOWN,
        LEFT | UP,
        RIGHT | UP,
    ];

    const DIRS: [(i32, i32, u8, u8); 4] = [
        (-1, 0, UP, DOWN),
        (1, 0, DOWN, UP),
        (0, -1, LEFT, RIGHT),
        (0, 1, RIGHT, LEFT),
    ];

    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];
    q.push_back((0, 0));
    while let Some((r, c)) = q.pop_front() {
        if r == n - 1 && c == m - 1 {
            return true;
        }
        let target = grid[r][c] as usize;
        for (dr, dc, out_dir, back_dir) in DIRS {
            if out_dir & MASK[target] == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= n as i32 || nc < 0 || nc >= m as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if visited[nr][nc] {
                continue;
            }
            let n_target = grid[nr][nc] as usize;
            if back_dir & MASK[n_target] == 0 {
                continue;
            }
            visited[nr][nc] = true;
            q.push_back((nr, nc));
        }
    }
    false
}
