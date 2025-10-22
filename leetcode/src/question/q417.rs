use std::collections::VecDeque;

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = heights.len();
    if m == 0 { return vec![]; }
    let n = heights[0].len();
    if n == 0 { return vec![]; }

    // 建立四向移動
    let dirs = [(1,0), (-1,0), (0,1), (0,-1)];

    // 小工具：從多個起點做一次 BFS（只走到高度 >= 當前高度的格子）
    let mut bfs = |starts: Vec<(usize, usize)>| -> Vec<Vec<bool>> {
        let mut vis = vec![vec![false; n]; m];
        let mut q = VecDeque::new();

        for (r, c) in starts {
            if !vis[r][c] {
                vis[r][c] = true;
                q.push_back((r, c));
            }
        }

        while let Some((r, c)) = q.pop_front() {
            let h = heights[r][c];
            for (dr, dc) in dirs.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize { continue; }
                let (nr, nc) = (nr as usize, nc as usize);
                if vis[nr][nc] { continue; }
                // 反向流動的條件：下一格高度 >= 當前高度
                if heights[nr][nc] >= h {
                    vis[nr][nc] = true;
                    q.push_back((nr, nc));
                }
            }
        }
        vis
    };

    // 太平洋：上邊界、左邊界
    let mut pac_starts = Vec::with_capacity(m + n);
    for c in 0..n { pac_starts.push((0, c)); }
    for r in 0..m { pac_starts.push((r, 0)); }
    let pac = bfs(pac_starts);

    // 大西洋：下邊界、右邊界
    let mut atl_starts = Vec::with_capacity(m + n);
    for c in 0..n { atl_starts.push((m - 1, c)); }
    for r in 0..m { atl_starts.push((r, n - 1)); }
    let atl = bfs(atl_starts);

    // 兩者皆可達的格子即答案
    let mut ans = Vec::new();
    for r in 0..m {
        for c in 0..n {
            if pac[r][c] && atl[r][c] {
                ans.push(vec![r as i32, c as i32]);
            }
        }
    }
    ans
}