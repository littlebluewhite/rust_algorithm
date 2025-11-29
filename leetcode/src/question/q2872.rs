pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    let mut graph = vec![vec![]; n as usize];
    let mut ans = 0i32;
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        graph[u].push(v);
        graph[v].push(u);
    }
    fn dfs(
        u: usize,
        parent: usize,
        graph: &Vec<Vec<usize>>,
        values: &Vec<i32>,
        k: i64,
        ans: &mut i32,
    ) -> i64 {
        let mut sum = values[u] as i64;
        for &v in &graph[u] {
            if v == parent {
                continue;
            }
            sum += dfs(v, u, graph, values, k, ans)
        }
        if sum % k == 0 {
            *ans += 1;
            0
        } else {
            sum
        }
    }
    dfs(0, usize::MAX, &graph, &values, k, &mut ans);
    ans
}
