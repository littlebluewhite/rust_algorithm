use std::collections::VecDeque;
const MOD: i64 = 1_000_000_007;
pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as i64;
    let mut dp = vec![0; n + 1];
    let mut prefix = vec![0; n + 1];
    let mut min_q: VecDeque<usize> = VecDeque::new();
    let mut max_q: VecDeque<usize> = VecDeque::new();
    dp[0] = 1;
    prefix[0] = 1;
    let mut l = 0usize;
    for r in 0..n {
        let val = nums[r] as i64;
        while let Some(&idx) = max_q.back() {
            if nums[idx] as i64 <= val {
                max_q.pop_back();
            } else {
                break;
            }
        }
        max_q.push_back(r);
        while let Some(&idx) = min_q.back() {
            if nums[idx] as i64 >= val {
                min_q.pop_back();
            } else {
                break;
            }
        }
        min_q.push_back(r);
        while l <= r
            && nums[*max_q.front().unwrap()] as i64 - nums[*min_q.front().unwrap()] as i64 > k
        {
            if max_q.front().copied() == Some(l) {
                max_q.pop_front();
            }
            if min_q.front().copied() == Some(l) {
                min_q.pop_front();
            }
            l += 1;
        }
        let prefix_left = if l == 0 { 0 } else { prefix[l - 1] };
        dp[r + 1] = (prefix[r] - prefix_left + MOD) % MOD;
        prefix[r + 1] = (prefix[r] + dp[r + 1]) % MOD;
    }
    dp[n] as i32
}
