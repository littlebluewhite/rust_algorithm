pub fn minimum_changes(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let k = k as usize;
    let inf = n as i32 + 1;
    let mut divisors = vec![vec![]; n + 1];
    for d in 1..=n / 2 {
        for len in (d * 2..=n).step_by(d) {
            divisors[len].push(d);
        }
    }
    let mut cost = vec![vec![inf; n + 1]; n];
    for len in 2..=n {
        for left in 0..=n - len {
            for &d in &divisors[len] {
                let group_len = len / d;
                let mut c = 0;
                for offset in 0..d {
                    for pair in 0..group_len / 2 {
                        let a = left + offset + pair * d;
                        let b = left + offset + (group_len - 1 - pair) * d;
                        if bytes[a] != bytes[b] {
                            c += 1;
                        }
                    }
                }
                cost[left][left + len] = cost[left][left + len].min(c);
            }
        }
    }
    let mut dp = vec![vec![inf; n + 1]; k + 1];
    dp[0][0] = 0;
    for parts in 1..=k {
        for end in 2 * parts..=n {
            for split in 2 * (parts - 1)..=end - 2 {
                if dp[parts - 1][split] != inf {
                    dp[parts][end] = dp[parts][end].min(dp[parts - 1][split] + cost[split][end]);
                }
            }
        }
    }
    dp[k][n]
}
