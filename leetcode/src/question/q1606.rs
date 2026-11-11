use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
    let k = k as usize;
    let n = arrival.len();
    let mut counts = vec![0;k];
    let mut max_times = 0;
    let mut free: BTreeSet<usize> = (0..k).collect();
    let mut busy: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

    for i in 0..n {
        let now = arrival[i];
        while let Some(&Reverse((end, server))) = busy.peek() {
            if end <= now {
                busy.pop();
                free.insert(server);
            } else {
                break;
            }
        }

        let start = i % k;
        let Some(&server) = free.range(start..).next().or_else(||free.first()) else {
            continue;
        };
        free.remove(&server);

        counts[server] += 1;
        max_times = max_times.max(counts[server]);
        busy.push(Reverse((now + load[i], server)));
    }


    let mut result = Vec::new();
    for i in 0..k {
        if counts[i] == max_times {
            result.push(i as i32);
        }
    }

    result
}