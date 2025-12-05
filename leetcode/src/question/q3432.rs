pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut right = 0;
    let mut left:i32 = nums.iter().sum();
    let mut ans = 0;
    for i in 0..nums.len()-1{
        left -= nums[i];
        right += nums[i];
        if (left - right)%2 ==0{
            ans += 1;
        }
    }
    ans
}