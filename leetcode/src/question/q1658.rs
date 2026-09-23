pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let n = nums.len();
    let total: i64 = nums.iter().map(|&x| x as i64).sum();
    let target = total - x as i64;
    if target < 0 {
        return -1;
    }
    if target == 0 {
        return n as i32;
    }
    let mut left = 0usize;
    let mut best = -1;
    let mut sum = 0;
    for right in 0..n {
        sum += nums[right] as i64;
        while sum > target && left <= right {
            sum -= nums[left] as i64;
            left += 1;
        }
        if sum == target{
            best = best.max((right - left + 1)as i32);
        }
    }
    if best == -1 {
        return -1;
    }
    n as i32 - best
}
