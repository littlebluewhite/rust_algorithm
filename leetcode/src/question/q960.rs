pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let m = strs[0].len();
    let mut dp = vec![1; m];
    let strs_b: Vec<Vec<u8>> = strs.iter().map(|s| s.as_bytes().to_vec()).collect();
    for i in 0..m{
        for j in 0..i{
            let mut ok = true;
            for k in 0..n{
                if strs_b[k][j] > strs_b[k][i]{
                    ok = false;
                    break
                }
            }
            if ok{
                dp[i] = dp[i].max(dp[j]+1)
            }
        }
    }
    let max = *dp.iter().max().unwrap();
    m as i32 -max
}