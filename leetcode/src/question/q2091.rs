pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut min_index = 0;
    let mut max_index = 0;
    for i in 0..n {
        if nums[i] < nums[min_index] {
            min_index = i;
        }
        if nums[i] > nums[max_index] {
            max_index = i;
        }
    }
    let n = n as i32;
    let left = min_index.min(max_index) as i32;
    let right = min_index.max(max_index) as i32;
    let front_only = right + 1;
    let back_only = n - left;
    let split = left + 1 + n - right;
    front_only.min(back_only).min(split)
}
