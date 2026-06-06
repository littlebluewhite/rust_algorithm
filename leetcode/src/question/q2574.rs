pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = Vec::with_capacity(n);
    let mut left_sum = Vec::with_capacity(nums.len());
    left_sum.push(0);
    let mut sum = 0;
    for i in 0..n-1{
        sum += nums[i];
        left_sum.push(sum);
    }
    sum += nums[n-1];
    for i in 0..n{
        sum -= nums[i];
        ans.push((left_sum[i] - sum).abs());
    }
    ans
}