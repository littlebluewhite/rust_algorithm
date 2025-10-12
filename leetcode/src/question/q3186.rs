use std::collections::HashMap;

pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
    let mut count: HashMap<i32, i64> = HashMap::new();
    for p in power {
        *count.entry(p).or_insert(0i64) += p as i64
    }
    let mut keys: Vec<i32> = count.keys().cloned().collect();
    keys.sort();
    let n = keys.len();
    let mut dp = vec![0i64; n];
    for i in 0..n {
        let val = keys[i];
        let take = count[&val];
        let mut j = i as i32 - 1;
        while j >= 0 {
            if keys[j as usize] <= val - 3 {
                break;
            } else {
                j -= 1
            }
        }
        let choose = if j >= 0 { take + dp[j as usize] } else { take };
        dp[i] = choose.max(dp[i]);
        if i > 0 {
            dp[i] = dp[i].max(dp[i - 1]);
        }
    }
    dp[n - 1]
}
