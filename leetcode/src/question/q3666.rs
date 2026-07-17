use std::collections::{BTreeSet, VecDeque};

pub fn min_operations(s: String, k: i32) -> i32 {
    let n = s.len() as i32;
    let start_zeros = s.as_bytes().iter().filter(|&&c| c == b'0').count();
    if start_zeros == 0 {
        return 0;
    }

    let mut event_set: BTreeSet<i32> = BTreeSet::new();
    let mut odd_set: BTreeSet<i32> = BTreeSet::new();
    for i in 0..=n {
        if i % 2 == 0 {
            event_set.insert(i);
        } else {
            odd_set.insert(i);
        }
    }
    let mut dp = vec![-1; n as usize + 1];
    let mut q: VecDeque<i32> = VecDeque::new();
    dp[start_zeros] = 0;
    if start_zeros % 2 == 0 {
        event_set.remove(&(start_zeros as i32));
    } else {
        odd_set.remove(&(start_zeros as i32));
    }
    q.push_back(start_zeros as i32);
    while let Some(m) = q.pop_front() {
        // k-c <= n-m
        let c1 = (k - n + m).max(0);
        let c2 = k.min(m);
        if c1 > c2 {
            continue;
        }
        let left = m + k - 2 * c2;
        let right = m + k - 2 * c1;
        let target_set = if left % 2 == 0 {
            &mut event_set
        } else {
            &mut odd_set
        };
        let next_nodes: Vec<i32> = target_set.range(left..=right).copied().collect();
        for next in next_nodes {
            target_set.remove(&next);
            dp[next as usize] = dp[m as usize] + 1;
            if next == 0 {
                return dp[0];
            }
            q.push_back(next);
        }
    }
    dp[0]
}
