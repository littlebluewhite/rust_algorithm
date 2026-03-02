use std::collections::{BTreeSet, VecDeque};

pub fn min_operations(s: String, k: i32) -> i32 {
    let n = s.len() as i32;
    let start_zero = s.bytes().filter(|&c| c == b'0').count() as i32;
    let mut dist: Vec<i32> = vec![-1; (n + 1) as usize];
    let mut odd_unvisited: BTreeSet<i32> = BTreeSet::new();
    let mut even_unvisited: BTreeSet<i32> = BTreeSet::new();
    for i in 0..=n {
        if i % 2 == 0 {
            even_unvisited.insert(i);
        } else {
            odd_unvisited.insert(i);
        }
    }
    if start_zero % 2 == 0 {
        even_unvisited.remove(&start_zero);
    } else {
        odd_unvisited.remove(&start_zero);
    }
    dist[start_zero as usize] = 0;
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(start_zero);
    while let Some(m) = q.pop_front() {
        let d = dist[m as usize];
        let c_up = k.min(m);
        let c_down = (k + m - n).max(0);
        if c_up < c_down {
            continue;
        }

        let left = m + k - 2 * c_up;
        let right = m + k - 2 * c_down;

        let target_v = if left % 2 == 0 {
            &mut even_unvisited
        } else {
            &mut odd_unvisited
        };
        let next_nodes: Vec<i32> = target_v.range(left..=right).copied().collect();
        for &next in &next_nodes {
            target_v.remove(&next);
            dist[next as usize] = d + 1;
            q.push_back(next);
        }
    }

    dist[0]
}
