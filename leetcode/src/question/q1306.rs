use std::collections::VecDeque;

pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let n = arr.len();
    let start = start as usize;
    let mut visited = vec![false; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(start);
    while let Some(i) = q.pop_front() {
        if arr[i] == 0 {
            return true;
        }
        visited[i] = true;
        let next_1 = i + arr[i] as usize;
        let next_2 = i as isize - arr[i] as isize;
        if next_1 < n && !visited[next_1]{
            q.push_back(next_1);
        }
        if next_2 >= 0 && !visited[next_2 as usize]{
            q.push_back(next_2 as usize);
        }
    }
    false
}