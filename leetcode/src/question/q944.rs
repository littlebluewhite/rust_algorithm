pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let n = strs[0].len();
    let strs_b: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
    let mut ans = 0;
    for i in 0..n{
        let mut pre = 0;
        for &str in &strs_b{
            if str[i] < pre{
                ans += 1;
                break
            }
            pre = str[i];
        }
    }
    ans
}