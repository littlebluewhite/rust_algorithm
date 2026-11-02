use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
    let labels = labels.as_bytes();
    let n = n as usize;
    let k = k as usize;
    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for edge in edges {
        adj[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }
    let width = k+1;
    let mut dist = vec![i32::MAX;n*width];
    let mut heap = BinaryHeap::new();
    dist[1] = 0;
    heap.push(Reverse((0i32, 0usize, 1usize)));
    while let Some(Reverse((cost, u, run))) = heap.pop(){
        if dist[(u*width)+run] != cost {
            continue;
        }
        if u==n-1{
            return cost;
        }
        for &(v, w) in &adj[u] {
            let next_run = if labels[v] == labels[u]{run+1}else{1};
            if next_run>k{
                continue;
            }
            let next_cost = cost + w;
            if next_cost < dist[(v*width)+next_run] {
                dist[(v*width)+next_run] = next_cost;
                heap.push(Reverse((next_cost, v, next_run)));
            }
        }
    }
    -1
}