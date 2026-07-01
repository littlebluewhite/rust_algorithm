use std::collections::{BinaryHeap, VecDeque};

pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut dist = vec![vec![-1; m]; n];
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                q.push_back((i as i32, j as i32));
                dist[i][j] = 0;
            }
        }
    }
    while let Some(node) = q.pop_front() {
        for dir in dirs.iter() {
            let nr = node.0 + dir.0;
            let nc = node.1 + dir.1;
            if nr < 0
                || nr >= n as i32
                || nc < 0
                || nc >= m as i32
                || dist[nr as usize][nc as usize] != -1
            {
                continue;
            }
            dist[nr as usize][nc as usize] = dist[node.0 as usize][node.1 as usize] + 1;
            q.push_back((nr, nc));
        }
    }
    let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    heap.push((dist[0][0], 0, 0));
    let mut visited = vec![vec![false;m];n];

    while let Some((safe, r, c)) = heap.pop(){
        if visited[r as usize][c as usize] {
            continue;
        }
        visited[r as usize][c as usize] = true;
        if r == n as i32 - 1 && c == m as i32 - 1 {
            return safe;
        }
        for dir in dirs.iter(){
            let nr = r+dir.0;
            let nc = c+dir.1;
            if nr < 0 || nr >= n as i32 || nc < 0 || nc >= m as i32 || visited[nr as usize][nc as usize] {
                continue;
            }
            let new_safe = safe.min(dist[nr as usize][nc as usize]);
            heap.push((new_safe, nr, nc));
        }
    }
    0
}
