// Input: n = 4, edges = [[0,1,3],[3,1,1],[2,3,4],[0,2,2]]
// Output: 5

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let inf = i64::MAX / 4;
    let mut dist = vec![inf; n];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        let w = edge[2] as i64;
        adj[u].push((v, w));
        adj[v].push((u, w * 2));
    }
    dist[0] = 0;
    heap.push(Reverse((0, 0)));
    while let Some(Reverse((s, index))) = heap.pop() {
        if s != dist[index] {
            continue;
        }
        if index == n - 1 {
            break;
        }
        for &(next, w) in &adj[index]{
            let new_cost = s + w;
            if new_cost < dist[next] {
                dist[next] = new_cost;
                heap.push(Reverse((new_cost, next)));
            }
        }
    }
    if dist[n - 1] == inf {
        return -1;
    }
    dist[n - 1]
}
