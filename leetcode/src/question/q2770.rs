pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut dp = vec![-1; n];
    dp[0] = 0;
    for j in 1..n{
        for i in 0..j{
            if dp[i] == -1{
                continue;
            }
            if (nums[j] - nums[i]).abs() <= target{
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }
    }
    dp[n-1]
}