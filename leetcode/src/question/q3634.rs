pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    let mut left = 0usize;
    let mut max_len = 1usize;
    for right in 0..n {
        if nums[right] as i64 > nums[left] as i64 * k as i64 {
            left += 1
        }
        max_len = max_len.max(right - left + 1)
    }
    (n - max_len) as i32
}
