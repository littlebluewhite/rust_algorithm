pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    let mut max = i32::MIN;
    let mut suffix_min = vec![i32::MAX; nums.len() + 1];
    let n = nums.len();
    for i in (0..n).rev() {
        suffix_min[i] = suffix_min[i + 1].min(nums[i]);
    }
    for i in 0..n {
        max = nums[i].max(max);
        if max - suffix_min[i] <= k {
            return i as i32;
        }
    }
    -1
}
