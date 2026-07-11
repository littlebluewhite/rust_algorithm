use std::collections::VecDeque;

pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1]as usize);
        graph[edge[1] as usize].push(edge[0]as usize);
    }
    let mut count = 0;
    let mut visited: Vec<bool> = vec![false; n];
    for start in 0..n{
        if visited[start]{
            continue;
        }
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(start);
        let mut degree_sum = 0;
        let mut vertices = 0;
        visited[start] = true;
        while let Some(v) = q.pop_front() {
            vertices += 1;
            degree_sum += graph[v].len();
            for &next in &graph[v] {
                if !visited[next] {
                    visited[next] = true;
                    q.push_back(next);
                }
            }
        }
        if vertices * (vertices - 1) == degree_sum {
            count += 1
        }
    }
    count
}