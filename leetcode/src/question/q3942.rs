pub fn min_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ascending = true;
    let mut descending = true;
    let mut zero_pos = 0usize;
    for i in 0..n {
        let value = nums[i] as usize;
        let next = nums[(i + 1) % n] as usize;
        ascending &= next == (value + 1) % n;
        descending &= next == (value + n - 1) % n;
        if value == 0 {
            zero_pos = i;
        }
    }
    let mut ans = usize::MAX;
    if ascending {
        ans = ans.min(zero_pos.min(n - zero_pos + 2));
    }
    if descending {
        ans = ans.min((zero_pos + 2).min(n - zero_pos))
    }
    if ans == usize::MAX { -1 } else { ans as i32 }
}
