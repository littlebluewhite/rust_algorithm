use std::collections::VecDeque;
const MOD: i64 = 1000000007;

pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len() + 1;
    let mut graph = vec![Vec::<usize>::new(); n];
    for edge in edges {
        let u = edge[0] as usize - 1;
        let v = edge[1] as usize - 1;
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back((0, usize::MAX, 0));
    let mut max_depth = 0;
    while let Some((node, parent, depth)) = q.pop_front() {
        if depth > max_depth {
            max_depth = depth;
        }
        for &next in &graph[node] {
            if next == parent{
                continue;
            }
            q.push_back((next, node, depth + 1));
        }
    }
    mod_pow(2, max_depth as i64 - 1) as i32
}
fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut ans = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            ans = (ans * base) % MOD;
        }
        base = (base * base) % MOD;
        exp >>= 1;
    }
    ans
}
