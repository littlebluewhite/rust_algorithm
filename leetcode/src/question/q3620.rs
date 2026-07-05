use std::collections::VecDeque;

pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
    let n = online.len();
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    let mut costs = Vec::with_capacity(edges.len());
    let mut deg = vec![0i32; n];
    for edge in edges {
        let (u, v, c) = (edge[0] as usize, edge[1] as usize, edge[2]);
        adj[u].push((v, c as i64));
        costs.push(c as i64);
        deg[v] += 1;
    }
    if costs.is_empty() {
        return -1;
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if deg[i] == 0 {
            q.push_back(i);
        }
    }
    let mut topo = Vec::with_capacity(n);
    while let Some(u) = q.pop_front() {
        topo.push(u);
        for &(v, _) in &adj[u] {
            deg[v] -= 1;
            if deg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    costs.sort_unstable();
    costs.dedup();

    const INF: i64 = i64::MAX / 4;
    let feasible = |x: i64| -> bool {
        let mut dp = vec![INF; n];
        dp[n - 1] = 0;
        for &u in topo.iter().rev() {
            if u != n - 1 && !online[u] {
                continue;
            }
            for &(v, c) in &adj[u] {
                if c >= x && dp[v] < INF {
                    dp[u] = dp[u].min(dp[v] + c);
                }
            }
        }
        dp[0] <= k
    };

    let mut lo = 0usize;
    let mut hi = costs.len();
    let mut ans = -1;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if feasible(costs[mid]) {
            ans = costs[mid];
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    ans as i32
}
