pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut suffix_min = vec![i32::MAX; n + 1];
    for i in (0..n).rev() {
        suffix_min[i] = suffix_min[i + 1].min(nums[i]);
    }
    let mut res = vec![0; n];
    let mut start = 0usize;
    let mut pre_max = i32::MIN;
    for i in 0..n {
        pre_max = pre_max.max(nums[i]);
        if i == n - 1 || suffix_min[i + 1] >= pre_max {
            for j in start..=i {
                res[j] = pre_max;
            }
            start = i + 1;
            pre_max = i32::MIN;
        }
    }
    res
}
