pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    let mut ans = 0;
    for i in 0..n/2{
        let sum = nums[i] + nums[n-1-i];
        if sum > ans{
            ans = sum;
        }
    }
    ans
}