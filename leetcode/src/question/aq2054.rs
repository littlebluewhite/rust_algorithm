pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
    let n = events.len();
    events.sort_by(|a, b| a[0].cmp(&b[0]));
    let starts = events.iter().map(|e| e[0]).collect::<Vec<i32>>();
    let mut suffix_max = vec![0; n + 1];
    for i in (0..n).rev() {
        suffix_max[i] = suffix_max[i + 1].max(events[i][2]);
    }
    let mut ans = 0i32;
    for i in 0..n {
        let end = events[i][1];
        let next_start = end + 1;
        let val = events[i][2];
        let j = lower_bound(&starts, next_start);
        let total = if j < n { val + suffix_max[j] } else { val };
        if total > ans {
            ans = total;
        }
    }
    ans
}

fn lower_bound(starts: &[i32], target: i32) -> usize {
    let mut l = 0usize;
    let mut r = starts.len();
    while l < r {
        let m = (r + l) / 2;
        if starts[m] >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}
