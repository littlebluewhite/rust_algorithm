pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ranked: Vec<(i32, i32, usize)> = nums
        .into_iter()
        .enumerate()
        .map(|(i, v)| (digit_sum(v), v, i))
        .collect();
    ranked.sort_unstable();
    let mut target = vec![0;n];
    for (d, (_, _, idx)) in ranked.into_iter().enumerate() {
        target[idx] = d;
    }
    let mut ans = 0;
    let mut visited = vec![false; n];
    for i in 0..n{
        if visited[i]{
            continue;
        }
        let mut cycle_len = 0;
        let mut current = i;
        while !visited[current]{
            visited[current] = true;
            current = target[current];
            cycle_len += 1;
        }
        ans += cycle_len - 1;
    }
    ans
}

fn digit_sum(mut value: i32) -> i32 {
    let mut ans = 0;
    while value > 0 {
        ans += value % 10;
        value /= 10;
    }
    ans
}
