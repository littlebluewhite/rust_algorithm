pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let n = stone_value.len();
    let mut dp = vec![vec![0; n]; n + 1];
    for i in 0..n {
        dp[1][i] = stone_value[i];
    }
    for len in 2..=n {
        for l in 0..=n - len {
            let r = l + len - 1;
            let take_left = stone_value[l] - dp[len - 1][l + 1];
            let take_right = stone_value[r] - dp[len - 1][l];
            dp[len][l] = take_left.max(take_right);
        }
    }
    match dp[n][0] {
        0 => "Tie".to_string(),
        _ if dp[n][0] > 0 => "Alice".to_string(),
        _ => "Bob".to_string(),
    }
}
