use std::collections::{HashMap, VecDeque};

pub fn min_jumps(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }
    let mut map: HashMap<i32, Vec<usize>> = HashMap::with_capacity(n);
    for (i, &x) in arr.iter().enumerate() {
        map.entry(x).or_default().push(i);
    }
    let mut q: VecDeque<(usize, i32)> = VecDeque::with_capacity(n);
    let mut visited = vec![false; n];
    q.push_back((0, 0));
    while let Some((i, c)) = q.pop_front() {
        if i == n - 1 {
            return c;
        }
        visited[i] = true;
        if i>1 && !visited[i-1]{
            q.push_back((i-1, c+1));
        }
        if i<n-1 && !visited[i+1]{
            q.push_back((i+1, c+1));
        }
        if let Some(bucket) = map.remove(&arr[i]){
            for j in bucket{
                if j != i && !visited[j]{
                    visited[j] = true;
                    q.push_back((j, c+1));
                }
            }
        }
    }
    0
}