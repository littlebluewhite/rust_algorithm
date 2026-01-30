pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut points: Vec<(usize, usize)> = Vec::with_capacity(n * m);
    for i in 0..n {
        for j in 0..m {
            points.push((i, j));
        }
    }
    points.sort_by_key(|p| grid[p.0][p.1]);

    let mut costs = vec![vec![i32::MAX; m]; n];
    for _ in 0..=k {
        let mut i = 0usize;
        let mut j = 0usize;
        let mut min_cost = i32::MAX;
        while i < points.len() {
            min_cost = min_cost.min(costs[points[i].0][points[i].1]);
            if i + 1 < points.len()
                && grid[points[i].0][points[i].1] == grid[points[i + 1].0][points[i + 1].1]
            {
                i += 1;
                continue;
            }
            for idx in j..=i {
                costs[points[idx].0][points[idx].1] = min_cost;
            }
            i += 1;
            j = i;
        }
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if i == n - 1 && j == m - 1 {
                    costs[i][j] = 0;
                    continue;
                }
                if i < n - 1 {
                    costs[i][j] = costs[i][j].min(costs[i + 1][j] + grid[i + 1][j]);
                }
                if j < m - 1 {
                    costs[i][j] = costs[i][j].min(costs[i][j + 1] + grid[i][j + 1]);
                }
            }
        }
    }

    costs[0][0]
}
