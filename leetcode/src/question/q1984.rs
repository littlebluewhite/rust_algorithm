pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut min = i32::MAX;
    nums.sort_unstable();
    for i in 0..=nums.len() - k {
        min = min.min(nums[i + k - 1] - nums[i])
    }
    min
}
