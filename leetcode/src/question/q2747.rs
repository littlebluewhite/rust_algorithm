pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, mut queries: Vec<i32>) -> Vec<i32> {
    let mut orders: Vec<(i32, usize)> = queries
        .into_iter()
        .enumerate()
        .map(|(index, time)| (time, index))
        .collect();
    orders.sort_unstable();
    logs.sort_unstable_by_key(|log| log[1]);
    let mut ans = vec![0; orders.len()];
    let mut right = 0usize;
    let mut left = 0usize;
    let mut count = vec![0; n as usize];
    let mut active = 0;
    for (time, index) in orders {
        while right < logs.len() && logs[right][1] <= time {
            let server_index = logs[right][0] as usize - 1;
            if count[server_index] == 0 {
                active += 1;
            }
            count[server_index] += 1;
            right += 1;
        }
        while left < right && logs[left][1] < time - x {
            let server_index = logs[left][0] as usize - 1;
            count[server_index] -= 1;
            if count[server_index] == 0 {
                active -= 1;
            }
            left += 1;
        }
        ans[index] = n - active;
    }
    ans
}
