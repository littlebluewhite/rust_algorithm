pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let mut max = nums[0];
    let mut min = nums[0];
    for n in nums.into_iter().skip(1) {
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }
    }
    (max - min) as i64 * k as i64
}
