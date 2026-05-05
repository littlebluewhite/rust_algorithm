pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let total: i32 = nums.iter().sum();
    let mut current: i32 = nums.iter().enumerate().map(|(i, &x)| x * i as i32).sum();
    let mut ans = current;
    for i in 1..n {
        let moved = nums[n - i];
        current = current + total - moved * n as i32;
        ans = ans.max(current);
    }
    ans
}
