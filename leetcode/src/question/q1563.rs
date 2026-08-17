pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    let n = stone_value.len();
    if n <= 1 {
        return 0;
    }
    let mut dp = vec![vec![0i64; n]; n];
    let mut best_left = vec![vec![0; n]; n];
    let mut best_right = vec![vec![0; n]; n];
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + stone_value[i] as i64;
    }
    for i in 0..n {
        best_left[i][i] = stone_value[i] as i64;
        best_right[i][i] = stone_value[i] as i64;
    }
    for len in 2..=n {
        for l in 0..=n - len {
            let r = l + len - 1;
            let total = prefix[r + 1] - prefix[l];
            let mut best = 0i64;
            if let Some(k) = find_last_left(l, r, &prefix, total) {
                best = best.max(best_left[l][k]);
            };
            if let Some(s) = find_first_right(l, r, &prefix, total) {
                best = best.max(best_right[s][r]);
            }
            dp[l][r] = best;
            best_left[l][r] = best_left[l][r - 1].max(total + dp[l][r]);
            best_right[l][r] = best_right[l + 1][r].max(total + dp[l][r]);
        }
    }
    dp[0][n - 1] as i32
}

fn find_last_left(l: usize, r: usize, prefix: &Vec<i64>, total: i64) -> Option<usize> {
    let mut lo = l;
    let mut hi = r;
    let mut ans = None;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if (prefix[mid + 1] - prefix[l]) * 2 <= total {
            ans = Some(mid);
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    ans
}

fn find_first_right(l: usize, r: usize, prefix: &Vec<i64>, total: i64) -> Option<usize> {
    let mut lo = l + 1;
    let mut hi = r + 1;
    let mut ans = None;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if (prefix[r + 1] - prefix[mid]) * 2 <= total {
            ans = Some(mid);
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    ans
}
