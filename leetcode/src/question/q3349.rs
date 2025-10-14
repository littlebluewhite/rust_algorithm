pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    let k = k as usize;
    if 2 * k > n { return false; }

    // start[i] = 從 i 開始連續嚴格遞增的長度
    let mut start = vec![1usize; n];
    for i in (0..n-1).rev() {
        if nums[i] < nums[i + 1] {
            start[i] = start[i + 1] + 1;
        }
    }

    // 檢查是否存在 i，使得 [i..i+k-1] 與 [i+k..i+2k-1] 皆嚴格遞增
    for i in 0..=n - 2 * k {
        if start[i] >= k && start[i + k] >= k {
            return true;
        }
    }
    false
}