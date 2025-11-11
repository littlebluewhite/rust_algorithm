pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut stack = vec![0; nums.len()];
    let mut top = 0;
    let mut ans = 0;
    for &n in nums.iter() {
        while stack[top] > n {
            top -= 1;
        }
        if stack[top] != n {
            ans += 1;
            top += 1;
            stack[top] = n;
        }
    }
    ans
}
