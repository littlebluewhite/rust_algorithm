pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
    let n = arr.len();
    let mut dp = vec![0; n];
    let mut ans = 1;
    for i in 0..n {
        ans = ans.max(dfs(&arr, &mut dp, i, d));
    }
    ans
}

fn dfs(arr: &[i32], dp: &mut [i32], i: usize, d: i32) -> i32 {
    if dp[i] != 0 {
        return dp[i];
    }
    let mut ans = 1;
    for j in (i.saturating_sub(d as usize)..i).rev() {
        if arr[i] <= arr[j] {
            break;
        } else {
            ans = ans.max(dfs(arr, dp, j, d) + 1);
        }
    }
    for j in (i + 1)..(i + 1 + d as usize).min(arr.len()) {
        if arr[i] <= arr[j] {
            break;
        } else {
            ans = ans.max(dfs(arr, dp, j, d) + 1);
        }
    }
    dp[i] = ans;
    ans
}
