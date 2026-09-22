pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
    let n = nums.len();
    let k = k as usize;
    let mut dp = vec![0i64;k];
    let mut ans = vec![0i64; k];
    for i in 0..n {
        let num = nums[i] as i64;
        let idx = num as usize % k;
        let mut next = vec![0i64; k];
        next[idx] +=1;
        for j in 0..k{
            let new_idx = (idx * j) % k;
            next[new_idx] += dp[j];
        }
        for j in 0..k{
            ans[j] += next[j]
        }
        dp = next;
    }
    ans
}