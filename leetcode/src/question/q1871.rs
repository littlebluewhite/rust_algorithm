pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    if s[n-1] != b'0' {
        return false;
    }
    let min_jump = min_jump as usize;
    let max_jump = max_jump as usize;

    let mut dp = vec![false; n];
    dp[0] = true;
    let mut count = 0;
    for i in 1..n{
        if i>=min_jump && dp[i-min_jump]{
            count += 1;
        }
        if i>max_jump && dp[i-max_jump-1]{
            count -= 1;
        }
        dp[i] = count > 0 && s[i] == b'0';
    }
    dp[n-1]
}