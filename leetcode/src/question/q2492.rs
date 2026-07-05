use std::collections::VecDeque;

pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut adj = vec![vec![]; n as usize];
    for road in roads {
        let (u, v, c) = (road[0] as usize - 1, road[1] as usize - 1, road[2]);
        adj[u].push((v, c));
        adj[v].push((u, c));
    }
    let mut q = VecDeque::new();
    let mut visited = vec![false; n as usize];
    q.push_back(0);
    visited[0] = true;
    let mut ans = i32::MAX;
    while let Some(node) = q.pop_front() {
        for &(v, c) in &adj[node] {
            ans = ans.min(c);
            if !visited[v] {
                q.push_back(v);
                visited[v] = true;
            }
        }
    }
    ans
}
