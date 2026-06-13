use std::collections::VecDeque;

const MOD: i64 = 1000000007;
pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len() + 1;
    let mut graph = vec![Vec::new(); n + 1];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut log = 1usize;
    while (1 << log) <= n {
        log += 1;
    }
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut visited = vec![false; n + 1];
    let mut depth = vec![0usize; n + 1];
    let mut up = vec![vec![0; n + 1]; log];
    visited[1] = true;
    up[0][1] = 1;
    q.push_back(1usize);
    while let Some(node) = q.pop_front() {
        for &next in &graph[node] {
            if visited[next] {
                continue;
            }
            visited[next] = true;
            depth[next] = depth[node] + 1;
            up[0][next] = node;
            q.push_back(next);
        }
    }

    for i in 1..log {
        for j in 1..=n {
            up[i][j] = up[i - 1][up[i - 1][j]];
        }
    }

    let mut pow_2 = vec![1i64; n + 1];
    for i in 1..n {
        pow_2[i] = pow_2[i - 1] * 2 % MOD;
    }

    let mut ans = Vec::with_capacity(queries.len());
    for query in queries {
        let u = query[0] as usize;
        let v = query[1] as usize;
        let ancestor = lca(u, v, &up, &depth);
        let dist = depth[u] + depth[v] - 2 * depth[ancestor];
        if dist == 0 {
            ans.push(0);
        } else {
            ans.push(pow_2[dist-1] as i32);
        }
    }
    ans
}

fn lca(mut u: usize, mut v: usize, up: &[Vec<usize>], depth: &[usize]) -> usize {
    if depth[u] < depth[v] {
        std::mem::swap(&mut u, &mut v);
    }
    let diff = depth[u] - depth[v];
    for bit in 0..up.len() {
        if ((diff >> bit) & 1) == 1 {
            u = up[bit][u];
        }
    }
    if u == v {
        return u;
    }

    for bit in (0..up.len()).rev() {
        if up[bit][u] != up[bit][v] {
            u = up[bit][u];
            v = up[bit][v];
        }
    }
    up[0][u]
}
