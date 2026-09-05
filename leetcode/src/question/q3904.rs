pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut suffix_min: Vec<i32> = vec![i32::MAX; n + 1];
    for i in (0..n).rev() {
        suffix_min[i] = suffix_min[i + 1].min(nums[i]);
    }
    let mut max = i32::MIN;
    for i in 0..n {
        max = max.max(nums[i]);
        if max-suffix_min[i]<= k {
            return i as i32
        }
    }
    -1
}