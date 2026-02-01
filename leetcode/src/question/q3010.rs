pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
    let mut ans = nums[0];
    nums = nums[1..].to_vec();
    nums.sort_unstable();
    ans += nums[0];
    ans += nums[1];
    ans
}