pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
    let n = events.len();
    events.sort_by_key(|v| v[0]);
    let mut suffix_max = vec![0; n + 1];
    for i in (0..n).rev() {
        suffix_max[i] = suffix_max[i + 1].max(events[i][2]);
    }
    // println!("events: {:?}", events);
    // println!("suffix_max: {:?}", suffix_max);
    let starts: Vec<i32> = events.iter().map(|v| v[0]).collect();
    let mut best = 0;
    for event in events {
        let end = event[1];
        let val = event[2];
        let next_start = end + 1;
        // println!("event: {:?}", event);
        let j = lower_bound(&starts, next_start);
        // println!("j: {}", j);
        let sum_val = val + suffix_max[j];
        if sum_val > best {
            best = sum_val
        }
    }
    best
}

fn lower_bound(starts: &[i32], target: i32) -> usize {
    let mut l = 0;
    let mut r = starts.len();
    while l < r {
        let m = (l + r) / 2;
        if starts[m] >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}
