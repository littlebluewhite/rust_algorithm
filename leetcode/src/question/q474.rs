pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for str in strs {
        let (mut zeros, mut ones) = (0, 0);
        for b in str.bytes() {
            if b == b'0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        for i in (zeros..=m).rev() {
            for j in (ones..=n).rev() {
                dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
            }
        }
    }
    dp[m][n]
}
