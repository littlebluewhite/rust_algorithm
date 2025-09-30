pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
    while nums.len() > 1 {
        let mut next = Vec::with_capacity(nums.len() - 1);
        for i in 0..nums.len() - 1 {
            next.push((nums[i] + nums[i + 1]) % 10);
        }
        nums = next;
    }
    nums[0]
}
