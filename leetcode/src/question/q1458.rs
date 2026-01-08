pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let m = nums2.len();
    let mut dp = vec![vec![0; m]; n];
    for i in 0..n{
        for j in 0..m{
            let multi = nums1[i]*nums2[j];
            let mut best = multi;
            if j > 0{
                best = best.max(dp[i][j-1]);
            }
            if i > 0{
                best = best.max(dp[i-1][j]);
            }
            if i > 0 && j > 0{
                best = best.max(dp[i-1][j-1]+multi);
            }
            dp[i][j] = best;
        }
    }
    dp[n-1][m-1]
}