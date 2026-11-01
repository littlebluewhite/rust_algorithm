pub fn min_moves(nums: Vec<i32>) -> i32 {
    let mut min = i64::MAX;
    let n = nums.len() as i64;
    let mut total = 0i64;
    for value in nums {
        let value = value as i64;
        min = min.min(value);
        total += value;
    }
    (total - min * n) as i32
}
