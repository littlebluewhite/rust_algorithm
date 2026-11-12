pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut best = 1;
    let mut current_up = 1;
    let mut current_down = 1;

    for i in 1..nums.len() {
        if nums[i] > nums[i-1] {
            current_up += 1;
            current_down = 1;
        } else if nums[i] < nums[i-1] {
            current_down += 1;
            current_up = 1;
        } else {
            current_up = 1;
            current_down = 1;
        }
        best = best.max(current_down).max(current_up);
    }
    best
}