pub fn stone_game(piles: Vec<i32>) -> bool {
    let n = piles.len();
    let mut dp = vec![vec![0; n]; n+1];
    for i in 0..n{
        dp[1][i] = piles[i];
    }
    for len in 2..=n{
        for l in 0..=n - len{
            let r = l + len - 1;
            let take_left = piles[l] - dp[len-1][l+1];
            let take_right = piles[r] - dp[len-1][l];
            dp[len][l] = take_left.max(take_right);
        }
    }
    dp[n][0] > 0
}