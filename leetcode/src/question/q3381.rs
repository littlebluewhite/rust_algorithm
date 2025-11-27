pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut prefix = 0i64;
    let mut min_pre = vec![i64::MAX;k];
    min_pre[0] = 0;
    let mut ans = i64::MIN;
    for (idx, &num) in nums.iter().enumerate(){
        prefix += num as i64;
        let idx_mod = (idx+1) % k;
        if min_pre[idx_mod] != i64::MAX {
            ans = ans.max(prefix - min_pre[idx_mod]);
        }
        if prefix < min_pre[idx_mod] {
            min_pre[idx_mod] = prefix;
        }
    }
    ans
}
