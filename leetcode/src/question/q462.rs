pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let median = nums[n / 2];
    let mut ans = 0;
    for v in nums.into_iter() {
        ans += (v - median).abs();
    }
    ans
}
