pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
    nums.sort_by(|&a, &b| a.abs().cmp(&b.abs()));
    let n=nums.len();
    let is_odd = n%2==1;
    let mut ans = 0i64;
    for i in 0..n/2{
        ans+= nums[n-i-1] as i64 * nums[n-i-1] as i64;
        ans-= nums[i] as i64 * nums[i] as i64;

    }
    if is_odd {
        ans += nums[n/2] as i64 * nums[n/2] as i64;
    }
    ans
}