use std::collections::VecDeque;

pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let k = k as usize;
    let mut graph = vec![vec![]; n];
    for invocation in &invocations{
        let u = invocation[0] as usize;
        let v = invocation[1] as usize;
        graph[u].push(v);
    }
    let mut suspicious = vec![false; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(k);
    suspicious[k as usize] = true;
    while let Some(node) = q.pop_front(){
        for &child in &graph[node]{
            if suspicious[child] {
                continue;
            }
            suspicious[child] = true;
            q.push_back(child);
        }
    }
    for node in 0..n{
        for &child in &graph[node] {
            if suspicious[child] != suspicious[node]{
                return (0..n).map(|i| i as i32).collect()
            }
        }
    }
    (0..n).filter(|&i| !suspicious[i]).map(|i| i as i32).collect()
}