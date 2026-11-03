pub fn min_moves(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let max = nums.iter().max().unwrap();
    let mut ans = 0;
    for i in 0..n {
        ans += (nums[i] - max).abs();
    }
    ans
}