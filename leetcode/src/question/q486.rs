pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut dp: Vec<i32> = nums.iter().map(|&x| x).collect();
    for len in 2..=n {
        for l in 0..n - len + 1 {
            let r = l + len - 1;
            let take_left = nums[l] - dp[l + 1];
            let take_right = nums[r] - dp[l];
            dp[l] = take_left.max(take_right);
        }
    }
    dp[0] >= 0
}
